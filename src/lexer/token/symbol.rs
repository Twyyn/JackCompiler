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
    Gt,
    Lt,
    Equal,
    Tilde,
}

impl Symbol {
    #[must_use]
    pub fn from_byte(b: u8) -> Option<Self> {
        match b {
            b'{' => Some(Self::LeftBrace),
            b'}' => Some(Self::RightBrace),
            b'(' => Some(Self::LeftParen),
            b')' => Some(Self::RightParen),
            b'[' => Some(Self::LeftBracket),
            b']' => Some(Self::RightBracket),
            b'.' => Some(Self::Dot),
            b',' => Some(Self::Comma),
            b';' => Some(Self::Semicolon),
            b'+' => Some(Self::Plus),
            b'-' => Some(Self::Minus),
            b'*' => Some(Self::Star),
            b'/' => Some(Self::Slash),
            b'&' => Some(Self::Ampersand),
            b'|' => Some(Self::Pipe),
            b'>' => Some(Self::Lt),
            b'<' => Some(Self::Gt),
            b'=' => Some(Self::Equal),
            b'~' => Some(Self::Tilde),
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
            Self::Lt => '>',
            Self::Gt => '<',
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
            Symbol::Gt => Some(BinaryOp::Gt),
            Symbol::Lt => Some(BinaryOp::Lt),
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
