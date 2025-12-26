use std::{
    fs::{File, OpenOptions},
    io::{self, Stderr, Stdout, Write},
    path::{Path, PathBuf},
};

use colored::{Color, Colorize};
use env_logger::Target;
use log::{Level, LevelFilter};

struct MultiWriter {
    err: Option<Stderr>,
    out: Option<Stdout>,
    fd: File,
}

impl Write for MultiWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if let Some(err) = &mut self.err {
            err.write_all(buf)?;
        }

        if let Some(out) = &mut self.out {
            out.write(buf)?;
        }
        self.fd.write_all(buf)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        if let Some(err) = &mut self.err {
            err.flush()?;
        }

        if let Some(out) = &mut self.out {
            out.flush()?;
        }
        self.fd.flush()?;
        Ok(())
    }
}

pub struct StaplesLogger {
    stdout: bool,
    stderr: bool,
    colors: bool,
    log_file_path: Option<PathBuf>,
    log_level: LevelFilter,
}

fn get_color(level: Level) -> Color {
    match level {
        Level::Debug => Color::Cyan,
        Level::Info => Color::Green,
        Level::Warn => Color::Yellow,
        Level::Error => Color::Red,
        Level::Trace => Color::Magenta,
    }
}

impl Default for StaplesLogger {
    fn default() -> Self {
        Self {
            stdout: false,
            stderr: false,
            colors: false,
            log_file_path: None,
            log_level: LevelFilter::Warn,
        }
    }
}

impl StaplesLogger {
    pub fn new() -> Self {
        StaplesLogger::default()
    }

    pub fn with_stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    pub fn with_stderr(mut self) -> Self {
        self.stderr = true;
        self
    }

    pub fn with_colors(mut self) -> Self {
        self.colors = true;
        self
    }

    pub fn with_log_file<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.log_file_path = Some(path.as_ref().to_path_buf());
        self
    }

    pub fn with_log_level(mut self, level: LevelFilter) -> Self {
        self.log_level = level;
        self
    }

    pub fn start(&self) {
        let mut builder = env_logger::Builder::new();

        if let Some(file_path) = &self.log_file_path {
            let log_fd = OpenOptions::new()
                .create(true)
                .append(true)
                .open(file_path)
                .unwrap();
            let multi_writer = MultiWriter {
                err: Some(io::stderr()),
                out: None,
                fd: log_fd,
            };
            builder.target(env_logger::Target::Pipe(Box::new(multi_writer)));
        } else {
            builder.target(Target::Stderr);
        }

        let colors = self.colors;

        builder.filter_level(self.log_level);
        builder
            .format(move |buf, record| {
                let now_ms = chrono::Local::now().timestamp_millis();
                let now_sec = now_ms / 1000;
                let now_ms = now_ms - (now_sec * 1000);

                let target = match record.line() {
                    Some(v) => format!("{}:{v}", record.target()),
                    None => record.target().to_string(),
                };

                let msg = format!(
                    "{}.{:03} :: {:<5} :: {:<45} {}",
                    now_sec,
                    now_ms,
                    record.level(),
                    target,
                    record.args()
                );

                if colors {
                    let color = get_color(record.level());
                    writeln!(buf, "{}", msg.color(color))
                } else {
                    writeln!(buf, "{msg}")
                }
            })
            .init();
    }
}
