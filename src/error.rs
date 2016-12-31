use std;
use rusqlite;
use getopts;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    GtfsMapError(String),
    GetoptsFail(getopts::Fail),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Rusqlite(ref err) => err.description(),
            Error::GtfsMapError(ref err) => err.as_ref(),
            Error::GetoptsFail(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Rusqlite(ref err) => Some(err),
            Error::GtfsMapError(ref err) => None,
            Error::GetoptsFail(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Rusqlite(ref err) => write!(f, "SQLite error: {}", err),
            Error::GtfsMapError(ref err) => write!(f, "GtfsMap error: {}", err),
            Error::GetoptsFail(ref err) => write!(f, "Getopts error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Error {
        Error::Rusqlite(err)
    }
}

impl From<getopts::Fail> for Error {
    fn from(err: getopts::Fail) -> Error {
        Error::GetoptsFail(err)
    }
}
