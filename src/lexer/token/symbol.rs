use std::fmt;

use crate::parser::ast::BinaryOp;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Symbol {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Dot,
    Comma,
    Semicolon,
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,
    Pipe,
    GreaterThan,
    LessThan,
    Equal,
    Tilde,
}

impl Symbol {
    #[must_use]
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '{' => Some(Self::LeftBrace),
            '}' => Some(Self::RightBrace),
            '(' => Some(Self::LeftParen),
            ')' => Some(Self::RightParen),
            '[' => Some(Self::LeftBracket),
            ']' => Some(Self::RightBracket),
            '.' => Some(Self::Dot),
            ',' => Some(Self::Comma),
            ';' => Some(Self::Semicolon),
            '+' => Some(Self::Plus),
            '-' => Some(Self::Minus),
            '*' => Some(Self::Star),
            '/' => Some(Self::Slash),
            '&' => Some(Self::Ampersand),
            '|' => Some(Self::Pipe),
            '>' => Some(Self::GreaterThan),
            '<' => Some(Self::LessThan),
            '=' => Some(Self::Equal),
            '~' => Some(Self::Tilde),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::LeftBrace => '{',
            Self::RightBrace => '}',
            Self::LeftParen => '(',
            Self::RightParen => ')',
            Self::LeftBracket => '[',
            Self::RightBracket => ']',
            Self::Dot => '.',
            Self::Comma => ',',
            Self::Semicolon => ';',
            Self::Plus => '+',
            Self::Minus => '-',
            Self::Star => '*',
            Self::Slash => '/',
            Self::Ampersand => '&',
            Self::Pipe => '|',
            Self::GreaterThan => '>',
            Self::LessThan => '<',
            Self::Equal => '=',
            Self::Tilde => '~',
        }
    }

    #[must_use]
    pub fn as_binary_operation(self) -> Option<BinaryOp> {
        match self {
            Symbol::Plus => Some(BinaryOp::Add),
            Symbol::Minus => Some(BinaryOp::Sub),
            Symbol::Star => Some(BinaryOp::Mul),
            Symbol::Slash => Some(BinaryOp::Div),
            Symbol::Ampersand => Some(BinaryOp::And),
            Symbol::Pipe => Some(BinaryOp::Or),
            Symbol::GreaterThan => Some(BinaryOp::GreaterThan),
            Symbol::LessThan => Some(BinaryOp::LessThan),
            Symbol::Equal => Some(BinaryOp::Equal),
            _ => None,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
