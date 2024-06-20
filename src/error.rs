use bincode::error::{DecodeError, EncodeError};
use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    IOError(#[from] io::Error),
    #[error("{0}")]
    EncodeError(#[from] EncodeError),
    #[error("{0}")]
    DecodeError(#[from] DecodeError),
    #[error("{0}")]
    MissingDependency(String),
}

pub type Result<T> = std::result::Result<T, Error>;
