use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratorError {
    #[error("invalid CLI arguments error (expected {expected:?}, found {found:?}")]
    CliArgumentNoError {
        expected: u32,
        found: u32,
    },
    #[error("Read error")]
    ReadError { source: std::io::Error },
}
