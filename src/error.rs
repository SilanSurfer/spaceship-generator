use std::fmt;

#[derive(Debug)]
pub enum Error {
    WrongNumberOfArguments(u16),
    LackOfPartInTheFile(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::WrongNumberOfArguments(val) => write!(f, "Wrong number of arguments provided. Should be 2 got {}", val),
            Error::LackOfPartInTheFile(val) => write!(f, "Couldn't find part {} in provided file", val),
        }
    }
}