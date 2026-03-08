use super::SubroutineCall;

use std::fmt;

use crate::lexer::token::data_type::Identifier;

// --- Expression ---
#[derive(Debug, Clone, PartialEq)]
pub struct Expression {
    pub term: Term,
    pub operations: Vec<(BinaryOperation, Term)>,
}

impl Expression {
    #[must_use]
    pub fn new(term: Term, operations: Vec<(BinaryOperation, Term)>) -> Self {
        Self { term, operations }
    }
}

// --- Term ---
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Term {
    IntegerConstant(u16),
    StringConstant(Identifier),
    KeywordConstant(KeywordConstant),
    Variable(Identifier),
    ArrayAccess(Identifier, Box<Expression>),
    SubroutineCall(SubroutineCall),
    Grouped(Box<Expression>),
    Unary(UnaryOperation, Box<Term>),
}

// --- Keyword Constant ---

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}

// --- Operations ---

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    And,
    Or,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum UnaryOperation {
    Minus,
    Tilde,
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.term)?;
        for (operation, term) in &self.operations {
            write!(f, " {operation} {term}")?;
        }
        Ok(())
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerConstant(integer) => write!(f, "{integer}"),
            Self::StringConstant(string) => write!(f, "{string}"),
            Self::KeywordConstant(keyword) => write!(f, "{keyword}"),
            Self::Variable(variable) => write!(f, "{variable}"),
            Self::ArrayAccess(name, index) => write!(f, "{name}[{index}]"),
            Self::SubroutineCall(call) => match &call.receiver {
                Some(receiver) => write!(f, "{receiver}.{}(...)", call.name),
                None => write!(f, "{}(...)", call.name),
            },
            Self::Grouped(group) => write!(f, "({group})"),
            Self::Unary(unary_operation, term) => write!(f, "{unary_operation}{term}"),
        }
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
        write!(f, "{s}")
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::And => '&',
            Self::Or => '|',
            Self::GreaterThan => '>',
            Self::LessThan => '<',
            Self::Equal => '=',
        };
        write!(f, "{c}")
    }
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Minus => '-',
            Self::Tilde => '~',
        };
        write!(f, "{c}")
    }
}
