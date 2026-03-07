use super::{Expression, Statement};

// --- Type(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Int,
    Char,
    Boolean,
    Class(Box<str>),
}
// --- Return Type(Kind) ---
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnKind {
    Void,
    Kind(Kind),
}

// --- Class ---
#[derive(Debug)]
pub struct Class {
    pub name: Box<str>,
    pub variables: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarKind {
    Static,
    Field,
}
// --- Class Variable Declaration ---
#[derive(Debug)]
pub struct ClassVarDec {
    pub var_kind: ClassVarKind,
    pub kind: Kind,
    pub names: Vec<Box<str>>,
}

// --- Subroutine Type(Kind)---
#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

// --- Subroutine Declaration ---
#[derive(Debug)]
pub struct SubroutineDec {
    pub kind: SubroutineKind,
    pub return_kind: ReturnKind,
    pub name: Box<str>,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}
// --- Subroutine Call Declaration ---
#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall {
    pub name: Box<str>,
    pub receiver: Option<Box<str>>,
    pub arguments: Vec<Expression>,
}

// --- Parameter/Argument ---
#[derive(Debug)]
pub struct Parameter {
    pub kind: Kind,
    pub name: Box<str>,
}

// --- Subroutine Body ---
#[derive(Debug)]
pub struct SubroutineBody {
    pub variables: Vec<VarDec>,
    pub statements: Vec<Statement>,
}

// --- Variable Declaration ---
#[derive(Debug)]
pub struct VarDec {
    pub kind: Kind,
    pub names: Vec<Box<str>>,
}
