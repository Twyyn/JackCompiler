use crate::lexer::token::types::Identifier;

use super::{Expression, Statement};
use super::{fmt, xml_close_tag, xml_identifier, xml_keyword, xml_open_tag, xml_symbol};

// --- Class ---

#[derive(Debug)]
pub struct Class {
    pub name: Identifier,
    pub variables: Vec<ClassVarDec>,
    pub subroutines: Vec<SubroutineDec>,
}

// --- Class Variable Declaration ---

#[derive(Debug)]
pub struct ClassVarDec {
    pub names: Vec<Identifier>,
    pub type_: Type,
    pub variable_type: ClassVarType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarType {
    Static,
    Field,
}

// --- Variable Declaration ---

#[derive(Debug)]
pub struct VarDec {
    pub names: Vec<Identifier>,
    pub type_: Type,
}

// --- Subroutine Declaration ---

#[derive(Debug)]
pub struct SubroutineDec {
    pub name: Identifier,
    pub subroutine_type: SubroutineType,
    pub return_type: ReturnType,
    pub parameters: Vec<Parameter>,
    pub body: SubroutineBody,
}
// --- Subroutine Type---

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineType {
    Constructor,
    Function,
    Method,
}

// --- Subroutine Body ---

#[derive(Debug)]
pub struct SubroutineBody {
    pub variables: Vec<VarDec>,
    pub statements: Vec<Statement>,
}

// --- Subroutine Call Declaration ---

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall {
    pub name: Identifier,
    pub receiver: Option<Identifier>,
    pub arguments: Vec<Expression>,
}

// --- Jack Data Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Char,
    Boolean,
    Class(Identifier),
}

// --- Jack Return Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Void,
    Type(Type),
}

// --- Parameter/Argument ---

#[derive(Debug)]
pub struct Parameter {
    pub name: Identifier,
    pub type_: Type,
}

// --- Write XML Impls ---

impl Class {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "class", indent);

        xml_keyword(out, "class", indent + 1);
        xml_identifier(out, &self.name, indent + 1);
        xml_symbol(out, '{', indent + 1);

        for var in &self.variables {
            var.write_xml(out, indent + 1);
        }
        for sub in &self.subroutines {
            sub.write_xml(out, indent + 1);
        }

        xml_symbol(out, '}', indent + 1);
        xml_close_tag(out, "class", indent);
    }
}

impl ClassVarDec {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "classVarDec", indent);

        match self.variable_type {
            ClassVarType::Static => xml_keyword(out, "static", indent + 1),
            ClassVarType::Field => xml_keyword(out, "field", indent + 1),
        }

        self.type_.write_xml(out, indent + 1);

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                xml_symbol(out, ',', indent + 1);
            }
            xml_identifier(out, name, indent + 1);
        }

        xml_symbol(out, ';', indent + 1);
        xml_close_tag(out, "classVarDec", indent);
    }
}

impl VarDec {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "varDec", indent);
        xml_keyword(out, "var", indent + 1);
        self.type_.write_xml(out, indent + 1);

        for (i, name) in self.names.iter().enumerate() {
            if i > 0 {
                xml_symbol(out, ',', indent + 1);
            }
            xml_identifier(out, name, indent + 1);
        }

        xml_symbol(out, ';', indent + 1);
        xml_close_tag(out, "varDec", indent);
    }
}

impl SubroutineDec {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "subroutineDec", indent);

        match self.subroutine_type {
            SubroutineType::Constructor => xml_keyword(out, "constructor", indent + 1),
            SubroutineType::Function => xml_keyword(out, "function", indent + 1),
            SubroutineType::Method => xml_keyword(out, "method", indent + 1),
        }

        self.return_type.write_xml(out, indent + 1);
        xml_identifier(out, &self.name, indent + 1);

        xml_symbol(out, '(', indent + 1);
        write_parameter_list(out, &self.parameters, indent + 1);
        xml_symbol(out, ')', indent + 1);

        self.body.write_xml(out, indent + 1);

        xml_close_tag(out, "subroutineDec", indent);
    }
}

impl SubroutineBody {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        xml_open_tag(out, "subroutineBody", indent);
        xml_symbol(out, '{', indent + 1);

        for var in &self.variables {
            var.write_xml(out, indent + 1);
        }

        write_statements(out, &self.statements, indent + 1);

        xml_symbol(out, '}', indent + 1);
        xml_close_tag(out, "subroutineBody", indent);
    }
}

impl SubroutineCall {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        if let Some(receiver) = &self.receiver {
            xml_identifier(out, receiver, indent);
            xml_symbol(out, '.', indent);
        }

        xml_identifier(out, &self.name, indent);
        xml_symbol(out, '(', indent);
        write_expression_list(out, &self.arguments, indent);
        xml_symbol(out, ')', indent);
    }
}

impl Type {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        match self {
            Self::Int => xml_keyword(out, "int", indent),
            Self::Char => xml_keyword(out, "char", indent),
            Self::Boolean => xml_keyword(out, "boolean", indent),
            Self::Class(name) => xml_identifier(out, name, indent),
        }
    }
}

impl ReturnType {
    pub fn write_xml(&self, out: &mut String, indent: usize) {
        match self {
            Self::Void => xml_keyword(out, "void", indent),
            Self::Type(ty) => ty.write_xml(out, indent),
        }
    }
}

// --- Write XML Helpers ---

pub fn write_parameter_list(out: &mut String, params: &[Parameter], indent: usize) {
    xml_open_tag(out, "parameterList", indent);

    for (i, param) in params.iter().enumerate() {
        if i > 0 {
            xml_symbol(out, ',', indent + 1);
        }
        param.type_.write_xml(out, indent + 1);
        xml_identifier(out, &param.name, indent + 1);
    }

    xml_close_tag(out, "parameterList", indent);
}

pub fn write_expression_list(out: &mut String, exprs: &[Expression], indent: usize) {
    xml_open_tag(out, "expressionList", indent);

    for (i, expr) in exprs.iter().enumerate() {
        if i > 0 {
            xml_symbol(out, ',', indent + 1);
        }
        expr.write_xml(out, indent + 1);
    }

    xml_close_tag(out, "expressionList", indent);
}

pub fn write_statements(out: &mut String, stmts: &[Statement], indent: usize) {
    xml_open_tag(out, "statements", indent);

    for stmt in stmts {
        stmt.write_xml(out, indent + 1);
    }

    xml_close_tag(out, "statements", indent);
}

// --- Display Impls ---

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::new();
        self.write_xml(&mut out, 0);
        write!(f, "{out}")
    }
}

impl fmt::Display for ClassVarType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Field => write!(f, "field"),
            Self::Static => write!(f, "static"),
        }
    }
}

impl fmt::Display for SubroutineType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Constructor => write!(f, "constructor"),
            Self::Function => write!(f, "function"),
            Self::Method => write!(f, "method"),
        }
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Char => write!(f, "char"),
            Self::Boolean => write!(f, "boolean"),
            Self::Class(name) => write!(f, "{name}"),
        }
    }
}

impl fmt::Display for ReturnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Type(type_) => write!(f, "{type_}"),
        }
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.type_)
    }
}
