use std::fmt;

#[derive(Debug)]
pub enum SpaceshipError {
    WrongNumberOfArguments(u16),
    LackOfPartInTheFile(String),
}

impl std::error::Error for SpaceshipError {}

impl fmt::Display for SpaceshipError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpaceshipError::WrongNumberOfArguments(val) => write!(
                f,
                "Wrong number of arguments provided. Should be 2 got {}",
                val
            ),
            SpaceshipError::LackOfPartInTheFile(val) => {
                write!(f, "Couldn't find part {} in provided file", val)
            }
        }
    }
}
