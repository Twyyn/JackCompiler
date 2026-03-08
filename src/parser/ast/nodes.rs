use std::fmt;

use crate::lexer::token::data_type::Identifier;

use super::{Expression, Statement};

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
    pub type_: Type,
    pub variable_type: ClassVarType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarType {
    Static,
    Field,
}

// --- Subroutine Declaration ---
#[derive(Debug)]
pub struct SubroutineDec {
    pub name: Identifier,
    pub subroutine_type: SubroutineType,
    pub return_type: ReturnType,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}

// --- Subroutine Type---

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineType {
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
    pub arguments: Vec<Expression>,
}

// --- Jack Data Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(Identifier),
}
// --- Jack Return Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Void,
    Type(Type),
}

// --- Variable Declaration ---

#[derive(Debug)]
pub struct VarDec {
    pub type_: Type,
    pub names: Vec<Identifier>,
}

// --- Parameter/Argument ---

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Class: {} Variables: {} Subroutines: {}",
            self.name, self.variables, self.subroutines
        )
    }
}

impl fmt::Display for ClassVarDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Variable(s): {} Type: {} Variable Type: {}",
            self.names, self.type_, self.variable_type
        )
    }
}

impl fmt::Display for ClassVarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Field => "field",
            Self::Static => "static",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for SubroutineDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            " Subroutine: {} type: {} return type: {} parameters: {} body: {}",
            self.name, self.subroutine_type, self.return_type, self.parameters, self.body
        )
    }
}

impl fmt::Display for SubroutineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Constructor => "constructor",
            Self::Function => "function",
            Self::Method => "method",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for SubroutineBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Variable(s): {} Statement(s): {}",
            self.variables, self.statements
        )
    }
}

impl fmt::Display for SubroutineCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Call: {} Receiver: {} Argument(s): {}",
            self.name, self.receiver, self.arguments
        )
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Int => "int",
            Self::Char => "char",
            Self::Boolean => "boolean",
            Self::Class(_) => "class",
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Void => "void",
            Self::Type(type_) => &type_.to_string(),
        };
        write!(f, "{s}")
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parameter: {} Type: {}", self.name, self.type_)
    }
}
