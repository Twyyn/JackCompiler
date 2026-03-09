use super::SubroutineCall;
use super::fmt;

use crate::lexer::token::types::Identifier;

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
            Self::IntegerConstant(n) => write!(f, "{n}"),
            Self::StringConstant(s) => write!(f, "\"{s}\""),
            Self::KeywordConstant(k) => write!(f, "{k}"),
            Self::Variable(v) => write!(f, "{v}"),
            Self::ArrayAccess(name, index) => write!(f, "{name}[{index}]"),
            Self::SubroutineCall(call) => write!(f, "{call}"),
            Self::Grouped(expr) => write!(f, "({expr})"),
            Self::Unary(op, term) => write!(f, "{op}{term}"),
        }
    }
}

impl fmt::Display for KeywordConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::True => write!(f, "true"),
            Self::False => write!(f, "false"),
            Self::Null => write!(f, "null"),
            Self::This => write!(f, "this"),
        }
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
        match self {
            Self::Minus => write!(f, "-"),
            Self::Tilde => write!(f, "~"),
        }
    }
}
