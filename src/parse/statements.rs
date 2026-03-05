use super::{Expression, SubroutineCall};

// --- Statements ---

#[derive(Debug)]
pub enum Statement<'src> {
    Let(LetStatement<'src>),
    If(IfStatement<'src>),
    While(WhileStatement<'src>),
    Do(DoStatement<'src>),
    Return(ReturnStatement<'src>),
}

// --- Let ---

// Fix 4: private fields → public for consistency with the rest of the AST
#[derive(Debug)]
pub struct LetStatement<'src> {
    pub name: &'src str,
    pub index: Option<Expression<'src>>,
    pub expression: Expression<'src>,
}

impl<'src> LetStatement<'src> {
    pub fn new(
        name: &'src str,
        index: Option<Expression<'src>>,
        expression: Expression<'src>,
    ) -> Self {
        Self {
            name,
            index,
            expression,
        }
    }
}

// --- If ---

#[derive(Debug)]
pub struct IfStatement<'src> {
    pub condition: Expression<'src>,
    pub statements: Vec<Statement<'src>>,
    pub else_statements: Option<Vec<Statement<'src>>>,
}

impl<'src> IfStatement<'src> {
    pub fn new(
        condition: Expression<'src>,
        statements: Vec<Statement<'src>>,
        else_statements: Option<Vec<Statement<'src>>>,
    ) -> Self {
        Self {
            condition,
            statements,
            else_statements,
        }
    }
}

// --- While ---

#[derive(Debug)]
pub struct WhileStatement<'src> {
    pub condition: Expression<'src>,
    pub statements: Vec<Statement<'src>>,
}

impl<'src> WhileStatement<'src> {
    pub fn new(condition: Expression<'src>, statements: Vec<Statement<'src>>) -> Self {
        Self {
            condition,
            statements,
        }
    }
}

// --- Do ---

#[derive(Debug)]
pub struct DoStatement<'src> {
    pub subroutine: SubroutineCall<'src>,
}

impl<'src> DoStatement<'src> {
    pub fn new(subroutine: SubroutineCall<'src>) -> Self {
        Self { subroutine }
    }
}

// --- Return ---

#[derive(Debug)]
pub struct ReturnStatement<'src> {
    pub expression: Option<Expression<'src>>,
}

impl<'src> ReturnStatement<'src> {
    pub fn new(expression: Option<Expression<'src>>) -> Self {
        Self { expression }
    }
}
