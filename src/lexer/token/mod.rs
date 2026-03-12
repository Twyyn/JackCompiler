pub mod keyword;
pub mod kind;
pub mod symbol;

pub use keyword::Keyword;
pub use kind::TokenKind;
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.kind)
    }
}

// --- Span ---

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn new(start: usize, end: usize) -> Self {
        let start = start as u32;
        let end = end as u32;
        Self { start, end }
    }
}

// pub struct Span {
//     pub offset: u32,
//     pub len: u16,
//     pub line: u32,
//     pub column: u16,
// }

// impl Span {
//     #[must_use]
//     pub fn new(offset: u32, len: u16, line: u32, column: u16) -> Self {
//         Self {
//             offset,
//             len,
//             line,
//             column,
//         }
//     }
// }
