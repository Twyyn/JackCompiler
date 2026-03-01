use std::fmt;

use crate::token::Token;

#[derive(Debug)]
pub enum ParseError<'src> {
    InvalidToken(Token<'src>),
    UnexpectedToken(Token<'src>),
    UnexpectedEof,
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token) => write!(f, "invalid token{token}"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
        }
    }
}
