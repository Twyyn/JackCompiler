use std::fmt;

use crate::token::TokenError;

#[derive(Debug)]
pub enum CompilerError {
    InvalidPath,
    NoJackFiles,
    Io(std::io::Error),
    TokenError(TokenError),
}

impl From<TokenError> for CompilerError {
    fn from(error: TokenError) -> Self {
        CompilerError::TokenError(error)
    }
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        CompilerError::Io(error)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "path is not a valid .jack file or directory"),
            Self::NoJackFiles => write!(f, "no .jack files found in the provided directory"),
            Self::Io(error) => write!(f, "{error}"),
            Self::TokenError(token_error) => write!(f, "{token_error}"),
        }
    }
}
