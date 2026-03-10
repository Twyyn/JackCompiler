use std::fmt;

use crate::lexer::LexerError;
use crate::parser::error::ParseError;

pub type CompilerResult<T> = std::result::Result<T, CompilerError>;

#[derive(Debug)]
pub enum CompilerError {
    InvalidPath,
    NoJackFiles,
    Io(std::io::Error),
    Lexer(LexerError),
    Parse(ParseError),
}

impl From<std::io::Error> for CompilerError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<LexerError> for CompilerError {
    fn from(err: LexerError) -> Self {
        Self::Lexer(err)
    }
}

impl From<ParseError> for CompilerError {
    fn from(err: ParseError) -> Self {
        Self::Parse(err)
    }
}

impl fmt::Display for CompilerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidPath => write!(f, "path is not a valid .jack file or directory"),
            Self::NoJackFiles => write!(f, "no .jack files found in the provided directory"),
            Self::Io(error) => write!(f, "{error}"),
            Self::Lexer(error) => write!(f, "{error}"),
            Self::Parse(error) => write!(f, "{error}"),
        }
    }
}

impl std::error::Error for CompilerError {}
