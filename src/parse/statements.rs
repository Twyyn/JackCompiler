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
    name: &'src str,
    index: Option<u16>,
    expression: Expression<'src>,
}

impl<'src> LetStatement<'src> {
    pub fn new(name: &'src str, index: Option<u16>, expression: Expression<'src>) -> Self {
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
    condition: Expression<'src>,
    statements: Vec<Statement<'src>>,
    else_statements: Option<Vec<Statement<'src>>>,
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
    condition: Expression<'src>,
    statements: Vec<Statement<'src>>,
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
    subroutine: SubroutineCall<'src>,
}

impl<'src> DoStatement<'src> {
    pub fn new(subroutine: SubroutineCall<'src>) -> Self {
        Self { subroutine }
    }
}

// --- Return ---

#[derive(Debug)]
pub struct ReturnStatement<'src> {
    expression: Option<Expression<'src>>,
}

impl<'src> ReturnStatement<'src> {
    pub fn new(expression: Option<Expression<'src>>) -> Self {
        Self { expression }
    }
}

// impl fmt::Display for LetStatement<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match (self.name, self.index, self.expression.clone()) {
//             (name, Some(index), expression) => write!(f, "{name} {index} {expression}"),
//             (name, None, expression) => write!(f, "{name} {expression}"),
//         }
//     }
// }
