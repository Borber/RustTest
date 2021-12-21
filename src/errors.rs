use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaMieError {
    #[error("basic error")]
    IOError(#[from] io::Error),
    #[error("fail to: {0}")]
    RunSomethingError(String),
    #[error("unknown data store error")]
    Unknown,
}

pub fn rs_error(s: impl Into<String>) -> DaMieError{
    DaMieError::RunSomethingError(s.into())
}