// --- Expression ---

use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'src> {
    pub term: Term<'src>,
    pub operations: Option<Vec<(Operation, Term<'src>)>>,
}

impl<'src> Expression<'src> {
    pub fn new(term: Term<'src>, operations: Option<Vec<(Operation, Term<'src>)>>) -> Self {
        Self { term, operations }
    }
}

// --- Term ---

#[derive(Debug, Clone, PartialEq)]
pub enum Term<'src> {
    IntegerConstant(u16),
    StringConstant(&'src str),
    KeywordConstant(KeywordConstant),
    Variable(&'src str),
    Grouped(Box<Expression<'src>>),
    Unary(UnaryOperation, Box<Term<'src>>),
}

// --- Operations ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,
    Pipe,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
    Minus,
    Tilde,
}

// --- Keyword Constant ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Minus => '-',
            Self::Tilde => '~',
        };
        write!(f, "{c}");
        Ok(())
    }
}

impl fmt::Display for KeywordConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
            Self::This => "this",
        };
        write!(f, "{s}");
        Ok(())
    }
}

impl fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerConstant(integer) => write!(f, "{integer}"),
            Self::StringConstant(string) => write!(f, "{string}"),
            Self::KeywordConstant(keyword) => write!(f, "{keyword}"),
            Self::Variable(variable) => write!(f, "{variable}"),
            Self::Grouped(group) => write!(f, "{group}"),
            Self::Unary(unary_operation, term) => write!(f, "{unary_operation} {term}"),
        }
    }
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {:#?}", self.term, self.operations)
    }
}
