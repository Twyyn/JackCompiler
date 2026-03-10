use crate::lexer::token::types::Identifier;

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

// // --- Display Impls ---

// impl fmt::Display for Statement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for LetStatement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for IfStatement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for WhileStatement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for DoStatement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for ReturnStatement {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }
