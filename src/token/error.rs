use std::fmt;

use crate::JACK_INT_MAX;

#[derive(Debug)]
pub enum TokenError {
    /// Integer exceeds Jack's int max of 32767.
    IntegerOutOfRange(u32),
    /// Integer could not be parsed.
    InvalidInteger(String),
    InvalidSymbol(String),
    UnterminatedString,
}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerOutOfRange(int) => write!(
                f,
                "Integer {int} exceeds Jack's maximum value of {JACK_INT_MAX}"
            ),
            Self::InvalidInteger(src) => write!(f, "Invalid integer {src}"),
            Self::InvalidSymbol(src) => write!(f, "invalid symbol {src}"),
            Self::UnterminatedString => write!(f, "unterminated string literal"),
        }
    }
}
