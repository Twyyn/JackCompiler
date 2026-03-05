// --- Expression ---

use std::fmt;

use super::SubroutineCall;

#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'src> {
    pub term: Term<'src>,
    pub operations: Vec<(Operation, Term<'src>)>, // Fix 3: Option<Vec> → Vec
}

impl<'src> Expression<'src> {
    pub fn new(term: Term<'src>, operations: Vec<(Operation, Term<'src>)>) -> Self {
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
    ArrayAccess(&'src str, Box<Expression<'src>>),
    SubroutineCall(SubroutineCall<'src>), // Fix 5: was missing from enum
    Grouped(Box<Expression<'src>>),
    Unary(UnaryOperation, Box<Term<'src>>),
}

// --- Operations ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
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

impl fmt::Display for Operation {
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
        // Fix 7: Ok(write!(...)?) → write!(...)
        let c = match self {
            Self::Minus => '-',
            Self::Tilde => '~',
        };
        write!(f, "{c}")
    }
}

impl fmt::Display for KeywordConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Fix 7: Ok(write!(...)?) → write!(...)
        let s = match self {
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
            Self::This => "this",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Term<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IntegerConstant(integer) => write!(f, "{integer}"),
            Self::StringConstant(string) => write!(f, "{string}"),
            Self::KeywordConstant(keyword) => write!(f, "{keyword}"),
            Self::Variable(variable) => write!(f, "{variable}"),
            Self::ArrayAccess(name, index) => write!(f, "{name}[{index}]"),
            // Fix 5: SubroutineCall was missing from Display
            Self::SubroutineCall(call) => match call.receiver {
                Some(receiver) => write!(f, "{receiver}.{}(...)", call.name),
                None => write!(f, "{}(...)", call.name),
            },
            Self::Grouped(group) => write!(f, "({group})"),
            Self::Unary(unary_operation, term) => write!(f, "{unary_operation}{term}"),
        }
    }
}

impl fmt::Display for Expression<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Fix 3: operations is now Vec, fix Display to format properly
        write!(f, "{}", self.term)?;
        for (op, term) in &self.operations {
            write!(f, " {op} {term}")?;
        }
        Ok(())
    }
}
