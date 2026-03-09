use crate::lexer::token::types::Identifier;

use super::{Expression, Statement};
use super::{fmt, fmt_vector, pretty_list};

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

// --- Variable Declaration ---

#[derive(Debug)]
pub struct VarDec {
    pub names: Vec<Identifier>,
    pub type_: Type,
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

// --- Parameter/Argument ---

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let class = format!("Class: {}", self.name);
        writeln!(f, "{class}")?;
        writeln!(f)?;
        writeln!(f, "   Variables:")?;
        pretty_list(f, &self.variables, "   ")?;
        writeln!(f)?;
        writeln!(f, "   Subroutines:")?;
        pretty_list(f, &self.subroutines, "   ")?;
        writeln!(f)
    }
}

impl fmt::Display for ClassVarDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = fmt_vector(&self.names);
        write!(f, "{names}  ({}, {})", self.type_, self.variable_type)
    }
}

impl fmt::Display for ClassVarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field => write!(f, "field"),
            Self::Static => write!(f, "static"),
        }
    }
}

impl fmt::Display for VarDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let names = fmt_vector(&self.names);
        write!(f, "{names}: {}", self.type_)
    }
}

impl fmt::Display for SubroutineDec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let parameters = if self.parameters.is_empty() {
            "none".to_string()
        } else {
            fmt_vector(&self.parameters)
        };
        writeln!(
            f,
            "{} {} Parameters: ({parameters}) Return Type: {}",
            self.subroutine_type, self.name, self.return_type
        )?;
        for line in self.body.to_string().lines() {
            writeln!(f, "    {line}")?;
        }
        Ok(())
    }
}

impl fmt::Display for SubroutineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constructor => write!(f, "constructor"),
            Self::Function => write!(f, "function"),
            Self::Method => write!(f, "method"),
        }
    }
}

impl fmt::Display for SubroutineBody {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !self.variables.is_empty() {
            writeln!(f, "Locals:")?;
            pretty_list(f, &self.variables, "    ")?;
        }
        if !self.statements.is_empty() {
            writeln!(f, "Statements:")?;
            pretty_list(f, &self.statements, "    ")?;
        }
        Ok(())
    }
}

impl fmt::Display for SubroutineCall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let arguments = fmt_vector(&self.arguments);
        match &self.receiver {
            Some(receiver) => write!(f, "{receiver}.{}({arguments})", self.name),
            None => write!(f, "{}({arguments})", self.name),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Char => write!(f, "char"),
            Self::Boolean => write!(f, "boolean"),
            Self::Class(name) => write!(f, "{name}"),
        }
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Type(type_) => write!(f, "{type_}"),
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_)
    }
}
