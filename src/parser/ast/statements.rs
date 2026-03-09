use crate::lexer::token::types::Identifier;

use super::{Expression, SubroutineCall};
use super::{fmt, xml_close_tag, xml_identifier, xml_keyword, xml_open_tag, xml_symbol};

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

// --- Write XML Impls ---

impl Statement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        match self {
            Self::Let(s) => s.write_xml(out, indent),
            Self::If(s) => s.write_xml(out, indent),
            Self::While(s) => s.write_xml(out, indent),
            Self::Do(s) => s.write_xml(out, indent),
            Self::Return(s) => s.write_xml(out, indent),
        }
    }
}

impl LetStatement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "letStatement", indent);
        xml_keyword(out, "let", indent + 1);
        xml_identifier(out, &self.name, indent + 1);

        if let Some(index) = &self.index {
            xml_symbol(out, '[', indent + 1);
            index.write_xml(out, indent + 1);
            xml_symbol(out, ']', indent + 1);
        }

        xml_symbol(out, '=', indent + 1);
        self.expression.write_xml(out, indent + 1);
        xml_symbol(out, ';', indent + 1);
        xml_close_tag(out, "letStatement", indent);
    }
}

impl IfStatement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "ifStatement", indent);
        xml_keyword(out, "if", indent + 1);
        xml_symbol(out, '(', indent + 1);
        self.condition.write_xml(out, indent + 1);
        xml_symbol(out, ')', indent + 1);
        xml_symbol(out, '{', indent + 1);
        super::write_statements(out, &self.if_body, indent + 1);
        xml_symbol(out, '}', indent + 1);

        if let Some(else_body) = &self.else_body {
            xml_keyword(out, "else", indent + 1);
            xml_symbol(out, '{', indent + 1);
            super::write_statements(out, else_body, indent + 1);
            xml_symbol(out, '}', indent + 1);
        }

        xml_close_tag(out, "ifStatement", indent);
    }
}

impl WhileStatement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "whileStatement", indent);
        xml_keyword(out, "while", indent + 1);
        xml_symbol(out, '(', indent + 1);
        self.condition.write_xml(out, indent + 1);
        xml_symbol(out, ')', indent + 1);
        xml_symbol(out, '{', indent + 1);
        super::write_statements(out, &self.body, indent + 1);
        xml_symbol(out, '}', indent + 1);
        xml_close_tag(out, "whileStatement", indent);
    }
}

impl DoStatement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "doStatement", indent);
        xml_keyword(out, "do", indent + 1);
        self.subroutine_call.write_xml(out, indent + 1);
        xml_symbol(out, ';', indent + 1);
        xml_close_tag(out, "doStatement", indent);
    }
}

impl ReturnStatement {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "returnStatement", indent);
        xml_keyword(out, "return", indent + 1);

        if let Some(expr) = &self.expression {
            expr.write_xml(out, indent + 1);
        }

        xml_symbol(out, ';', indent + 1);
        xml_close_tag(out, "returnStatement", indent);
    }
}

// --- Display Impls ---

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for LetStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for IfStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for WhileStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for DoStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for ReturnStatement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}
