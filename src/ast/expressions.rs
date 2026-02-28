#[derive(Debug, Clone, PartialEq)]
pub struct Expression<'src> {
    pub term: Term<'src>,
    pub operations: Vec<(Operation, Term<'src>)>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Term<'src> {
    IntegerConstant(u32),
    StringConstant(&'src str),
    KeywordConstant(KeywordConstant),
    Variable(&'src str),
    Grouped(Box<Expression<'src>>),
    Unary(UnaryOperation, Box<Term<'src>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall<'src> {
    pub name: &'src str,
    pub receiver: Option<&'src str>,
    pub arguments: Vec<Expression<'src>>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,
    Pipe,
    GreaterThan,
    LessThan,
    Equal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperation {
    Minus,
    Tilde,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordConstant {
    True,
    False,
    Null,
    This,
}
