use std::fmt;
use std::num::ParseIntError;

use crate::JACK_INT_MAX;

#[derive(Debug)]
#[non_exhaustive]
pub enum LexerError {
    /// Integer exceeds Jack's int max of 32767.
    IntegerOutOfRange(u64),
    /// Integer could not be parsed.
    InvalidInteger(String, Option<ParseIntError>),
    InvalidSymbol(String),
    UnterminatedString,
    UnterminatedComment,
    CursorOutofBounds,
}

impl From<ParseIntError> for LexerError {
    fn from(err: ParseIntError) -> Self {
        LexerError::InvalidInteger(err.to_string(), Some(err))
    }
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerOutOfRange(int) => write!(
                f,
                "integer {int} exceeds Jack's maximum value of {JACK_INT_MAX}"
            ),
            Self::InvalidInteger(src, _) => write!(f, "invalid integer {src}"),
            Self::InvalidSymbol(src) => write!(f, "invalid symbol {src}"),
            Self::UnterminatedString => write!(f, "unterminated string literal"),
            Self::UnterminatedComment => write!(f, "unterminated comment"),
            Self::CursorOutofBounds => write!(f, "byte cursor out of bounds"),
        }
    }
}

impl std::error::Error for LexerError {}
