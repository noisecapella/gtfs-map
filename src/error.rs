use std;
use rusqlite;
use getopts;
use hyper;

#[derive(Debug)]
pub enum Error {
    Rusqlite(rusqlite::Error),
    GtfsMapError(String),
    GetoptsFail(getopts::Fail),
    Io(std::io::Error),
    Hyper(hyper::Error),
    ParseInt(std::num::ParseIntError),
    ParseFloat(std::num::ParseFloatError),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Rusqlite(ref err) => err.description(),
            Error::GtfsMapError(ref err) => err.as_ref(),
            Error::GetoptsFail(ref err) => err.description(),
            Error::Io(ref err) => err.description(),
            Error::Hyper(ref err) => err.description(),
            Error::ParseInt(ref err) => err.description(),
            Error::ParseFloat(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Rusqlite(ref err) => Some(err),
            Error::GtfsMapError(ref err) => None,
            Error::GetoptsFail(ref err) => Some(err),
            Error::Io(ref err) => Some(err),
            Error::Hyper(ref err) => Some(err),
            Error::ParseInt(ref err) => Some(err),
            Error::ParseFloat(ref err) => Some(err),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Rusqlite(ref err) => write!(f, "SQLite error: {}", err),
            Error::GtfsMapError(ref err) => write!(f, "GtfsMap error: {}", err),
            Error::GetoptsFail(ref err) => write!(f, "Getopts error: {}", err),
            Error::Io(ref err) => write!(f, "Io error: {}", err),
            Error::Hyper(ref err) => write!(f, "Hyper error: {}", err),
            Error::ParseInt(ref err) => write!(f, "Int parse error: {}", err),
            Error::ParseFloat(ref err) => write!(f, "Float parse error: {}", err),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Error {
        Error::Hyper(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::ParseInt(err)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(err: std::num::ParseFloatError) -> Error {
        Error::ParseFloat(err)
    }
}
