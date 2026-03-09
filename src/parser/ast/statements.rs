use crate::lexer::token::types::Identifier;

use super::{Expression, SubroutineCall};
use super::{fmt, pretty_list};

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

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Let(s) => write!(f, "{s}"),
            Self::If(s) => write!(f, "{s}"),
            Self::While(s) => write!(f, "{s}"),
            Self::Do(s) => write!(f, "{s}"),
            Self::Return(s) => write!(f, "{s}"),
        }
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Let {}", self.name)?;
        if let Some(index) = &self.index {
            write!(f, "[{index}]")?;
        }
        write!(f, " = {}", self.expression)
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "If ({}):", self.condition)?;
        pretty_list(f, &self.if_body, "    ")?;
        if let Some(else_body) = &self.else_body {
            writeln!(f, "Else:")?;
            pretty_list(f, else_body, "    ")?;
        }
        Ok(())
    }
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "While ({}):", self.condition)?;
        pretty_list(f, &self.body, "    ")
    }
}

impl fmt::Display for DoStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Do {}", self.subroutine_call)
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.expression {
            Some(expr) => write!(f, "Return {expr}"),
            None => write!(f, "Return"),
        }
    }
}
