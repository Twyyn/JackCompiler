use std::fmt;

use crate::lexer::{LexerError, token::Token};

#[derive(Debug)]
pub enum ParseError<'src> {
    InvalidToken(Token<'src>),
    UnexpectedToken(Token<'src>),
    UnexpectedEof,
    LexerError(LexerError),
}

impl<'src> From<LexerError> for ParseError<'src> {
    fn from(e: LexerError) -> Self {
        ParseError::LexerError(e)
    }
}

impl<'src> fmt::Display for ParseError<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidToken(token) => write!(f, "invalid token {token}"),
            Self::UnexpectedToken(token) => write!(f, "unexpected token {token}"),
            Self::UnexpectedEof => write!(f, "Unexpected EOF"),
            Self::LexerError(e) => write!(f, "{e}"),
        }
    }
}

impl std::error::Error for ParseError<'_> {}
