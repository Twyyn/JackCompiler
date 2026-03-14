use std::fmt;

use super::{Keyword, Symbol};


pub type Identifier = Box<str>;



#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum TokenKind {
    Keyword(Keyword),
    Symbol(Symbol),
    IntegerConstant(u32),
    StringConstant(Identifier),
    Identifier(Identifier),
    Eof,
}

impl fmt::Display for TokenKind {
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
