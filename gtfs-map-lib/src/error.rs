use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct XmlAttributeError {
    msg: String
}

impl XmlAttributeError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl std::error::Error for XmlAttributeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for XmlAttributeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "XmlAttributeError(msg = \"{}\")", self.msg);
        Ok(())
    }
}

#[derive(Debug)]
pub struct NoRouteError {
    msg: String
}

impl NoRouteError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl std::error::Error for NoRouteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for NoRouteError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoRouteError(msg = \"{}\")", self.msg);
        Ok(())
    }
}

#[derive(Debug)]
pub struct NoTripError {
    msg: String
}

impl NoTripError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl std::error::Error for NoTripError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for NoTripError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "NoTripError(msg = \"{}\")", self.msg);
        Ok(())
    }
}

#[derive(Debug)]
pub struct ArgumentError {
    msg: String
}

impl ArgumentError {
    pub fn new(msg: &str) -> Self {
        Self { msg: msg.to_string() }
    }
}

impl std::error::Error for ArgumentError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl std::fmt::Display for ArgumentError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "ArgumentError(msg = \"{}\")", self.msg);
        Ok(())
    }
}


#[derive(Debug)]
pub struct OtherError {
    msg: String
}

impl OtherError {
    pub fn new(msg: &str) -> Self {
        OtherError {
            msg: msg.to_string()
        }
    }
}

impl Display for OtherError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for OtherError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }

}