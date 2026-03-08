pub mod keyword;
pub mod symbol;
pub mod data_type;

pub use keyword::Keyword;
pub use symbol::Symbol;
pub use data_type::TokenType;

use std::fmt;

// --- Token ---

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub span: Span,
}

impl Token {
    #[must_use]
    pub fn new(token_type: TokenType, span: Span) -> Self {
        Self {
            token_type,
            span,
        }
    }

    #[must_use]
    pub fn lexeme<'src>(&self, source: &'src str) -> &'src str {
        let start = self.span.offset as usize;
        &source[start..start + self.span.len as usize]
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token_type)
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
