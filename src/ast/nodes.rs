pub use super::Statement;

// --- Types ---

#[derive(Debug, Clone, PartialEq)]
pub enum Kind<'src> {
    Int,
    Char,
    Boolean,
    Class(&'src str),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnKind<'src> {
    Void,
    Kind(Kind<'src>),
}

// --- Class ---

#[derive(Debug)]
pub struct Class<'src> {
    pub name: &'src str,
    pub variables: Vec<ClassVarDec<'src>>,
    pub subroutines: Vec<SubroutineDec<'src>>,
}

#[derive(Debug)]
pub struct ClassVarDec<'src> {
    pub var_kind: ClassVarKind,
    pub kind: Kind<'src>,
    pub names: Vec<&'src str>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassVarKind {
    Static,
    Field,
}

// --- Subroutine ---

#[derive(Debug)]
pub struct SubroutineDec<'src> {
    pub kind: SubroutineKind,
    pub return_kind: ReturnKind<'src>,
    pub name: &'src str,
    pub parameters: Vec<Parameter<'src>>,
    pub body: SubroutineBody<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

#[derive(Debug)]
pub struct Parameter<'src> {
    pub kind: Kind<'src>,
    pub name: &'src str,
}

#[derive(Debug)]
pub struct SubroutineBody<'src> {
    pub variables: Vec<VarDec<'src>>,
    pub statements: Vec<Statement<'src>>,
}

#[derive(Debug)]
pub struct VarDec<'src> {
    pub kind: Kind<'src>,
    pub names: Vec<&'src str>,
}
