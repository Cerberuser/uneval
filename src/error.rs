//! Error returned by Uneval serializer.

use serde::ser;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UnevalError {
    #[error("IO error while writing code: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization process yielded invalid UTF-8 sequence: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Unknown error: {0}")]
    Custom(String),
}

impl ser::Error for UnevalError {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
