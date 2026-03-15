use std::fmt;

use crate::lexer::token::Token;

#[derive(Debug)]
pub enum ParseError<'src> {
    InvalidToken(Token<'src>),
    UnexpectedToken(Token<'src>),
    UnexpectedEof,
}

impl<'src> fmt::Display for ParseError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token) => write!(f, "invalid token {token}"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
        }
    }
}

impl std::error::Error for ParseError<'_> {}
