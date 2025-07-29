pub type Result<T> = core::result::Result<T, Error>;

use std::path::PathBuf;

use derive_more::From;

#[derive(Debug, From)]
pub enum Error {
    //
    // 1st party
    //
    DirnameError,
    BasenameError,
    FileNotFoundError {
        path: PathBuf,
    },

    //
    // 2nd party
    //
    #[from]
    Io(std::io::Error),

    //
    // 3rd party
    //
    #[from]
    LoggingError(log::SetLoggerError),
}

impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error> {
        write!(fmt, "{self:?}")
    }
}
