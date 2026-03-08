use crate::lexer::token::r#type::Identifier;

use super::{Expression, Statement};

// --- Type(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum TypeKind {
    Int,
    Char,
    Boolean,
    Class(Identifier),
}
// --- Return Type(Kind) ---
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnTypeKind {
    Void,
    TypeKind(TypeKind),
}

// --- Class ---
#[derive(Debug)]
pub struct Class {
    pub name: Identifier,
    pub variables: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarTypeKind {
    Static,
    Field,
}
// --- Class Variable Declaration ---
#[derive(Debug)]
pub struct ClassVarDec {
    pub var_kind: ClassVarTypeKind,
    pub type_kind: TypeKind,
    pub names: Vec<Identifier>,
}

// --- Subroutine Type(Kind)---
#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineTypeKind {
    Constructor,
    Function,
    Method,
}

// --- Subroutine Declaration ---
#[derive(Debug)]
pub struct SubroutineDec {
    pub type_kind: SubroutineTypeKind,
    pub return_type_kind: ReturnTypeKind,
    pub name: Identifier,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}
// --- Subroutine Call Declaration ---
#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall {
    pub name: Identifier,
    pub receiver: Option<Identifier>,
    pub arguments: Vec<Expression>,
}

// --- Parameter/Argument ---
#[derive(Debug)]
pub struct Parameter {
    pub type_kind: TypeKind,
    pub name: Identifier,
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
    pub type_kind: TypeKind,
    pub names: Vec<Identifier>,
}
