use super::SubroutineCall;

use crate::lexer::token::kind::Identifier;

// --- Expression ---

#[derive(Debug, Clone, PartialEq)]
pub struct Expr {
    pub term: Term,
    pub op: Vec<(BinaryOp, Term)>,
}

impl Expr {
    #[must_use]
    pub fn new(term: Term, op: Vec<(BinaryOp, Term)>) -> Self {
        Self { term, op }
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
    ArrayAccess(Identifier, Box<Expr>),
    SubroutineCall(SubroutineCall),
    Grouped(Box<Expr>),
    Unary(UnaryOp, Box<Term>),
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

impl KeywordConstant {
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::True => "true",
            Self::False => "false",
            Self::Null => "null",
            Self::This => "this",
        }
    }
}

// --- Operations ---

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum BinaryOp {
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

impl BinaryOp {
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Mul => '*',
            Self::Div => '/',
            Self::And => '&',
            Self::Or => '|',
            Self::GreaterThan => '>',
            Self::LessThan => '<',
            Self::Equal => '=',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[non_exhaustive]
pub enum UnaryOp {
    Minus,
    Tilde,
}

impl UnaryOp {
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Minus => '-',
            Self::Tilde => '~',
        }
    }
}

// --- Display Impls ---

// impl fmt::Display for Expression {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for Term {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for KeywordConstant {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.as_str())
//     }
// }

// impl fmt::Display for BinaryOp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.as_char())
//     }
// }

// impl fmt::Display for UnaryOp {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.as_char())
//     }
// }
