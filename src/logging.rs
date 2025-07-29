use std::path::{Path, PathBuf};

use crate::error::Result;

#[derive(Default)]
pub struct StaplesLogger {
    verbose: bool,
    log_file_path: Option<PathBuf>,
}

impl StaplesLogger {
    pub fn new() -> Self {
        StaplesLogger::default()
    }

    pub fn with_stdout(mut self) -> Self {
        self.verbose = true;
        self
    }

    pub fn with_log_file<P>(mut self, path: P) -> Self
    where
        P: AsRef<Path>,
    {
        self.log_file_path = Some(path.as_ref().to_path_buf());
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
            .level(log::LevelFilter::Info);

        let f = match &self.log_file_path {
            Some(v) => f.chain(fern::log_file(v)?),
            None => f,
        };

        let f = match self.verbose {
            true => f.chain(std::io::stdout()),
            false => f,
        };

        f.apply()?;
        Ok(())
    }
}
