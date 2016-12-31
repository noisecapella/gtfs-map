struct GtfsMapError {

}

impl std::error::Error {
    fn description(&self) -> &str {
        "test"
    }

    fn cause(&self) -> Option<&Error> {
        None
    }
}

pub struct Error {
    BoxError(Box<std::error::Error>),
    GtfsMapError(&'static str),
}
