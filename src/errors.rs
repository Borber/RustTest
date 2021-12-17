use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DaMeiError {
    #[error("basic error")]
    IOError(#[from] io::Error),
    #[error("fail to: {0}")]
    RunSomethingError(String),
    #[error("command failed: {0}")]
    RunCommandError(String),
    #[error("unknown data store error")]
    Unknown,
}