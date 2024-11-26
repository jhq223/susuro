use thiserror::Error;

use crate::Rule;

#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("Parse failed at rule: {0:?}")]
    PestError(#[from] pest::error::Error<Rule>),

    #[error("Unexpected end of input")]
    UnexpectedEndOfInput,

    #[error("Unknown instruction encountered")]
    UnknownInstruction,

    #[error("Custom error: {0}")]
    Custom(String),
}

impl ParseError {
    pub fn custom<T: Into<String>>(msg: T) -> Self {
        ParseError::Custom(msg.into())
    }
}
