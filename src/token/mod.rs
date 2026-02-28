mod error;
mod keyword;
mod kind;
mod symbol;

pub use error::TokenError;
pub use keyword::Keyword;
pub use kind::{Identifier, TokenKind};
use std::fmt::{self};
pub use symbol::Symbol;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Token<'a> {
    pub kind: TokenKind<'a>,
    pub span: Span,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub offset: u32,
    pub len: u16,
    pub line: u32,
    pub column: u16,
}

impl<'src> Token<'src> {
    #[must_use]
    pub fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub fn lexeme(&self, source: &'src str) -> &'src str {
        let start = self.span.offset as usize;

        &source[start..start + self.span.len as usize]
    }
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

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}
