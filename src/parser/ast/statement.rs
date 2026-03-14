use super::{Expr, SubroutineCall};

// --- Statements ---

#[derive(Debug)]
pub enum Statement<'src> {
    Let(LetStmt<'src>),
    If(IfStmt<'src>),
    While(WhileStmt<'src>),
    Do(DoStmt<'src>),
    Return(ReturnStmt<'src>),
}

// --- Let ---

#[derive(Debug)]
pub struct LetStmt<'src> {
    pub name: &'src str,
    pub index: Option<Expr<'src>>,
    pub expr: Expr<'src>,
}

// --- If ---

#[derive(Debug)]
pub struct IfStmt<'src> {
    pub condition: Expr<'src>,
    pub if_body: Vec<Statement<'src>>,
    pub else_body: Option<Vec<Statement<'src>>>,
}

// --- While ---

#[derive(Debug)]
pub struct WhileStmt<'src> {
    pub condition: Expr<'src>,
    pub body: Vec<Statement<'src>>,
}

// --- Do ---

#[derive(Debug)]
pub struct DoStmt<'src> {
    pub subroutine_call: SubroutineCall<'src>,
}

// --- Return ---

#[derive(Debug)]
pub struct ReturnStmt<'src> {
    pub expr: Option<Expr<'src>>,
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
