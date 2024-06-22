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
    #[error("Required tables are missing")]
    MissingRequiredTable,
    #[error("Expected '{0}' table")]
    ExpectedTable(&'static str),
    #[error("Expected cmap subtable '{0}'")]
    UnsupportedCmapSubtable(u16),
}

pub type Result<T> = std::result::Result<T, Error>;
