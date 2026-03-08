use std::fmt;

use crate::lexer::token::Token;

#[derive(Debug)]
pub enum ParseError {
    InvalidToken(Token),
    UnexpectedToken(Token),
    UnexpectedEof,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token) => write!(f, "invalid token {token}"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
        }
    }
}

impl std::error::Error for ParseError {}
