use serde::de;
use std::fmt::Display;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("TrailingCharacters")]
    TrailingCharacters,
    #[error("Eof")]
    Eof,
    #[error("Expected boolean")]
    ExpectedBoolean,
    #[error("Expected integer")]
    ExpectedInteger,
    #[error("Expected boolean")]
    ExpectedString,
    #[error("{0}")]
    Custom(String),
    #[error("Invalid syntax")]
    Syntax,
    #[error("Expected null")]
    ExpectedNull,
    #[error("Expected array end")]
    ExpectedArrayEnd,
    #[error("Expected array")]
    ExpectedArray,
    #[error("Expected map end")]
    ExpectedMapEnd,
    #[error("Expected map")]
    ExpectedMap,
    #[error("Expected array comma")]
    ExpectedArrayComma,
    #[error("Expected enum")]
    ExpectedEnum,
    #[error("Expected map comma")]
    ExpectedMapComma,
    #[error("Expected map colon")]
    ExpectedMapColon,
}
impl de::Error for Error {
    fn custom<T: Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
