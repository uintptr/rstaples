use std::path::{Path, PathBuf};

use log::LevelFilter;

use crate::error::Result;

pub struct StaplesLogger {
    stdout: bool,
    stderr: bool,
    log_file_path: Option<PathBuf>,
    log_level: LevelFilter,
}

impl Default for StaplesLogger {
    fn default() -> Self {
        Self {
            stdout: false,
            stderr: false,
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

    pub fn start(&self) -> Result<()> {
        let f = fern::Dispatch::new()
            .format(|out, message, record| {
                let now_ms = chrono::Local::now().timestamp_millis();
                let now_sec = now_ms / 1000;
                let now_ms = now_ms - (now_sec * 1000);

                let target = match record.line() {
                    Some(v) => format!("{}:{v}", record.target()),
                    None => record.target().to_string(),
                };

                out.finish(format_args!(
                    "{}.{:03} :: {:<5} :: {:<45} {}",
                    now_sec,
                    now_ms,
                    record.level(),
                    target,
                    message
                ))
            })
            .level(self.log_level);

        let f = match &self.log_file_path {
            Some(v) => f.chain(fern::log_file(v)?),
            None => f,
        };

        let f = match self.stdout {
            true => f.chain(std::io::stdout()),
            false => f,
        };

        let f = match self.stdout {
            true => f.chain(std::io::stderr()),
            false => f,
        };

        f.apply()?;
        Ok(())
    }
}
