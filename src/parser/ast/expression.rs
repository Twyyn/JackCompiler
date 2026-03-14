use super::SubroutineCall;

// --- Expression ---

#[derive(Debug, Clone, PartialEq)]
pub struct Expr<'src> {
    pub term: Term<'src>,
    pub op: Vec<(BinaryOp, Term<'src>)>,
}

impl<'src> Expr<'src> {
    #[must_use]
    pub fn new(term: Term<'src>, op: Vec<(BinaryOp, Term<'src>)>) -> Self {
        Self { term, op }
    }
}

// --- Term ---

#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum Term<'src> {
    IntegerConstant(u16),
    StringConstant(&'src str),
    KeywordConstant(KeywordConstant),
    Variable(&'src str),
    ArrayAccess(&'src str, Box<Expr<'src>>),
    SubroutineCall(SubroutineCall<'src>),
    Grouped(Box<Expr<'src>>),
    Unary(UnaryOp, Box<Term<'src>>),
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
    Gt,
    Lt,
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
            Self::Gt => '>',
            Self::Lt => '<',
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
