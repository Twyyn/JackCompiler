use crate::lexer::token::r#type::Identifier;

use super::{Expression, SubroutineCall};

// --- Statements ---
#[derive(Debug)]
pub enum Statement {
    Let(LetStatement),
    If(IfStatement),
    While(WhileStatement),
    Do(DoStatement),
    Return(ReturnStatement),
}

// --- Let ---
#[derive(Debug)]
pub struct LetStatement {
    pub name: Identifier,
    pub index: Option<Expression>,
    pub expression: Expression,
}

// --- If ---
#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub if_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

// --- While ---
#[derive(Debug)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
}

// --- Do ---
#[derive(Debug)]
pub struct DoStatement {
    pub subroutine_call: SubroutineCall,
}

// --- Return ---
#[derive(Debug)]
pub struct ReturnStatement {
    pub expression: Option<Expression>,
}
