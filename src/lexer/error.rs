use std::fmt;

use crate::JACK_INT_MAX;


#[derive(Debug)]
#[non_exhaustive]
pub enum LexerError {
    /// Integer exceeds Jack's int max of 32767.
    IntegerOutOfRange(u64),
    /// Integer could not be parsed.
    InvalidInteger(String),
    InvalidSymbol(String),
    UnterminatedString,
    UnterminatedComment,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerOutOfRange(int) => write!(
                f,
                "integer {int} exceeds Jack's maximum value of {JACK_INT_MAX}"
            ),
            Self::InvalidInteger(src) => write!(f, "invalid integer {src}"),
            Self::InvalidSymbol(src) => write!(f, "invalid symbol {src}"),
            Self::UnterminatedString => write!(f, "unterminated string literal"),
            Self::UnterminatedComment => write!(f, "unterminated comment"),
        }
    }
}

impl std::error::Error for LexerError {}
