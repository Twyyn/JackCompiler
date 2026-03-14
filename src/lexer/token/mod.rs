pub mod keyword;
pub mod symbol;

pub use keyword::Keyword;
pub use symbol::Symbol;

use std::fmt;

// --- Token ---

#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    span: Span,
}

impl<'src> Token<'src> {
    pub fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn lexeme_as_bytes<'a>(&self, source: &'a [u8]) -> &'a [u8] {
        let start = self.span.start as usize;
        let end = start + self.span.len as usize;
        &source[start..end]
    }

    pub fn lexeme_as_str<'a>(&self, source: &'a str) -> &'a str {
        let start = self.span.start as usize;
        let end = start + self.span.len as usize;
        &source[start..end]
    }
}

// --- Token Kind ---

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind<'src> {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u32),
    StringConstant(&'src str),
    Identifier(&'src str),
    Eof,
}

impl<'src> TokenKind<'src> {
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(_))
    }

    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    pub fn is_intconstant(&self) -> bool {
        matches!(self, Self::IntegerConstant(_))
    }

    pub fn is_strconstant(&self) -> bool {
        matches!(self, Self::StringConstant(_))
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, Self::Eof)
    }
}

// --- Span ---

#[derive(Debug, Clone, Copy, PartialEq)]

pub struct Span {
    start: u32,
    len: u16,
}

impl Span {
    #[must_use]
    pub fn new(start: usize, len: usize) -> Self {
        debug_assert!(start <= u32::MAX as usize);
        debug_assert!(len <= u16::MAX as usize);
        Self {
            start: start as u32,
            len: len as u16,
        }
    }
}

// --- Impl Displays ---

impl<'src> fmt::Display for TokenKind<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(_) => f.write_str("keyword"),
            Self::Symbol(_) => f.write_str("symbol"),
            Self::IntegerConstant(_) => f.write_str("integerConstant"),
            Self::StringConstant(_) => f.write_str("stringConstant"),
            Self::Identifier(_) => f.write_str("identifier"),
            Self::Eof => f.write_str("EOF"),
        }
    }
}

impl<'src> fmt::Display for Token<'src> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.kind, f)
    }
}
