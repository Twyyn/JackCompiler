mod error;
mod keyword;
mod kind;
mod symbol;

pub use error::LexerError;
pub use keyword::Keyword;
pub use kind::{Identifier, TokenKind};
pub use symbol::Symbol;

use std::fmt;

// --- Token ---

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    #[must_use]
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub fn lexeme<'src>(&self, source: &'src str) -> &'src str {
        let start = self.span.offset as usize;
        &source[start..start + self.span.len as usize]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

// --- Span ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub offset: u32,
    pub len: u16,
    pub line: u32,
    pub column: u16,
}

impl Span {
    #[must_use]
    pub fn new(offset: u32, len: u16, line: u32, column: u16) -> Self {
        Self {
            offset,
            len,
            line,
            column,
        }
    }
}
