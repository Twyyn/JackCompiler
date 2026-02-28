use super::{Expression, SubroutineCall};

#[derive(Debug)]
pub enum Statement<'src> {
    Let(LetStatement<'src>),
    If(IfStatement<'src>),
    While(WhileStatement<'src>),
    Do(SubroutineCall<'src>),
    Return(Option<Expression<'src>>),
}

#[derive(Debug)]
pub struct LetStatement<'src> {
    name: &'src str,
    index: Option<u16>,
    expression: Expression<'src>,
}

#[derive(Debug)]
pub struct IfStatement<'src> {
    condition: Expression<'src>,
    statements: Vec<Statement<'src>>,
    else_statements: Option<Vec<Statement<'src>>>,
}

#[derive(Debug)]
pub struct WhileStatement<'src> {
    condition: Expression<'src>,
    statements: Vec<Statement<'src>>,
}
