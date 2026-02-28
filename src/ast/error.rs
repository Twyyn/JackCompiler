use crate::token::Token;
use std::fmt::{self};

#[derive(Debug)]
pub enum ParseError<'src> {
    InvalidToken(Token<'src>),
    UnexpectedToken(Token<'src>),
    UnexpectedEof,
}

impl fmt::Display for ParseError<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidToken(token) => write!(f, "invalid token{token}"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
        }
    }
}
