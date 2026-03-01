use std::fmt;

use crate::token::TokenError;

#[derive(Debug)]
pub enum CompilerError {
    InvalidPath,
    NoJackFiles,
    Io(std::io::Error),
    TokenError(TokenError),
}

impl From<std::io::Error> for CompilerError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<TokenError> for CompilerError {
    fn from(error: TokenError) -> Self {
        Self::TokenError(error)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "path is not a valid .jack file or directory"),
            Self::NoJackFiles => write!(f, "no .jack files found in the provided directory"),
            Self::Io(error) => write!(f, "{error}"),
            Self::TokenError(error) => write!(f, "{error}"),
        }
    }
}
