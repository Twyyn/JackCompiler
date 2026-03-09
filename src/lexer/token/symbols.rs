use std::fmt;

use crate::parser::ast::BinaryOperation;


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
    pub fn as_binary_operation(self) -> Option<BinaryOperation> {
        match self {
            Symbol::Plus => Some(BinaryOperation::Add),
            Symbol::Minus => Some(BinaryOperation::Sub),
            Symbol::Star => Some(BinaryOperation::Mul),
            Symbol::Slash => Some(BinaryOperation::Div),
            Symbol::Ampersand => Some(BinaryOperation::And),
            Symbol::Pipe => Some(BinaryOperation::Or),
            Symbol::GreaterThan => Some(BinaryOperation::GreaterThan),
            Symbol::LessThan => Some(BinaryOperation::LessThan),
            Symbol::Equal => Some(BinaryOperation::Equal),
            _ => None,
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
