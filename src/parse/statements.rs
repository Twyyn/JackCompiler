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
#[derive(Debug)]
pub struct LetStatement<'src> {
    pub name: &'src str,
    pub index: Option<Expression<'src>>,
    pub expression: Expression<'src>,
}

impl<'src> LetStatement<'src> {
    #[must_use]
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
    pub if_body: Vec<Statement<'src>>,
    pub else_body: Option<Vec<Statement<'src>>>,
}

impl<'src> IfStatement<'src> {
    #[must_use]
    pub fn new(
        condition: Expression<'src>,
        if_body: Vec<Statement<'src>>,
        else_body: Option<Vec<Statement<'src>>>,
    ) -> Self {
        Self {
            condition,
            if_body,
            else_body,
        }
    }
}

// --- While ---
#[derive(Debug)]
pub struct WhileStatement<'src> {
    pub condition: Expression<'src>,
    pub body: Vec<Statement<'src>>,
}

impl<'src> WhileStatement<'src> {
    #[must_use]
    pub fn new(condition: Expression<'src>, body: Vec<Statement<'src>>) -> Self {
        Self { condition, body }
    }
}

// --- Do ---
#[derive(Debug)]
pub struct DoStatement<'src> {
    pub subroutine: SubroutineCall<'src>,
}

impl<'src> DoStatement<'src> {
    #[must_use]
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
    #[must_use]
    pub fn new(expression: Option<Expression<'src>>) -> Self {
        Self { expression }
    }
}
