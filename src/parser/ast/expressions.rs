use super::SubroutineCall;
use super::{fmt, xml_close_tag, xml_keyword, xml_open_tag, xml_symbol, xml_terminal};

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

impl BinaryOperation {
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
pub enum UnaryOperation {
    Minus,
    Tilde,
}

impl UnaryOperation {
    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Self::Minus => '-',
            Self::Tilde => '~',
        }
    }
}

// --- Write XML Impls ---

impl Expression {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "expression", indent);
        self.term.write_xml(out, indent + 1);

        for (op, term) in &self.operations {
            xml_symbol(out, op.as_char(), indent + 1);
            term.write_xml(out, indent + 1);
        }

        xml_close_tag(out, "expression", indent);
    }
}

impl Term {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "term", indent);

        match self {
            Self::IntegerConstant(n) => {
                xml_terminal(out, "integerConstant", &n.to_string(), indent + 1);
            }
            Self::StringConstant(s) => {
                xml_terminal(out, "stringConstant", s, indent + 1);
            }
            Self::KeywordConstant(k) => {
                xml_keyword(out, k.as_str(), indent + 1);
            }
            Self::Variable(name) => {
                super::xml_identifier(out, name, indent + 1);
            }
            Self::ArrayAccess(name, index) => {
                super::xml_identifier(out, name, indent + 1);
                xml_symbol(out, '[', indent + 1);
                index.write_xml(out, indent + 1);
                xml_symbol(out, ']', indent + 1);
            }
            Self::SubroutineCall(call) => {
                call.write_xml(out, indent + 1);
            }
            Self::Grouped(expr) => {
                xml_symbol(out, '(', indent + 1);
                expr.write_xml(out, indent + 1);
                xml_symbol(out, ')', indent + 1);
            }
            Self::Unary(op, inner) => {
                xml_symbol(out, op.as_char(), indent + 1);
                inner.write_xml(out, indent + 1);
            }
        }

        xml_close_tag(out, "term", indent);
    }
}

// --- Display Impls ---

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for KeywordConstant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for BinaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}

impl fmt::Display for UnaryOperation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_char())
    }
}
