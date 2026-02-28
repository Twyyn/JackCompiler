use super::{Keyword, Symbol};
use std::fmt;

pub type Identifier<'a> = &'a str;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenKind<'a> {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u16),
    StringConstant(&'a str),
    Identifier(Identifier<'a>),
    Eof,
}

impl TokenKind<'_> {
    #[must_use]
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(_))
    }

    #[must_use]
    pub fn is_symbol(&self) -> bool {
        matches!(self, Self::Symbol(_))
    }

    #[must_use]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }
}

impl fmt::Display for TokenKind<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keyword(_) => write!(f, "keyword"),
            Self::Symbol(_) => write!(f, "symbol"),
            Self::IntegerConstant(_) => write!(f, "integerConstant"),
            Self::StringConstant(_) => write!(f, "stringConstant"),
            Self::Identifier(_) => write!(f, "identifier"),
            Self::Eof => write!(f, "EOF"),
        }
    }
}
