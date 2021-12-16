use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaMeiError {
    #[error("Basic error")]
    IOError(#[from] io::Error),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
    #[error("unknown data store error")]
    Unknown,
}