use crate::lexer::token::kind::Identifier;

use super::{Expr, Statement};

// --- Class ---

#[derive(Debug)]
pub struct Class {
    pub name: Identifier,
    pub variables: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

// --- Class Variable Declaration ---

#[derive(Debug)]
pub struct ClassVarDec {
    pub names: Vec<Identifier>,
    pub kind: DataKind,
    pub var_kind: ClassVarKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarKind {
    Static,
    Field,
}

// --- Variable Declaration ---

#[derive(Debug)]
pub struct VarDec {
    pub names: Vec<Identifier>,
    pub kind: DataKind,
}

// --- Subroutine Declaration ---

#[derive(Debug)]
pub struct SubroutineDec {
    pub name: Identifier,
    pub kind: SubroutineKind,
    pub return_kind: ReturnKind,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}
// --- Subroutine Types(Kind)---

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

// --- Subroutine Body ---

#[derive(Debug)]
pub struct SubroutineBody {
    pub variables: Vec<VarDec>,
    pub statements: Vec<Statement>,
}

// --- Subroutine Call Declaration ---

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall {
    pub name: Identifier,
    pub receiver: Option<Identifier>,
    pub args: Vec<Expr>,
}

// --- Data Types(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum DataKind {
    Int,
    Char,
    Boolean,
    Class(Identifier),
}

// --- Jack Return Types(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnKind {
    Void,
    Kind(DataKind),
}

// --- Parameter/Argument ---

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub kind: DataKind,
}

// impl fmt::Display for Class {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for ClassVarKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Field => write!(f, "field"),
//             Self::Static => write!(f, "static"),
//         }
//     }
// }

// impl fmt::Display for SubroutineKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Constructor => write!(f, "constructor"),
//             Self::Function => write!(f, "function"),
//             Self::Method => write!(f, "method"),
//         }
//     }
// }

// impl fmt::Display for DataKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Int => write!(f, "int"),
//             Self::Char => write!(f, "char"),
//             Self::Boolean => write!(f, "boolean"),
//             Self::Class(name) => write!(f, "{name}"),
//         }
//     }
// }

// impl fmt::Display for ReturnKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Void => write!(f, "void"),
//             Self::Kind(kind) => write!(f, "{kind}"),
//         }
//     }
// }

// impl fmt::Display for Parameter {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.name, self.kind)
//     }
// }
