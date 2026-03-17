use super::{Expr, Statement};

// --- Class ---

#[derive(Debug)]
pub struct Class<'src> {
    pub name: &'src str,
    pub variables: Vec<ClassVarDec<'src>>,
    pub subroutines: Vec<SubroutineDec<'src>>,
}

// --- Class Variable Declaration ---

#[derive(Debug)]
pub struct ClassVarDec<'src> {
    pub names: Vec<&'src str>,
    pub kind: Kind,
    pub ty: Type<'src>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Kind {
    Static,
    Field,
}

// --- Variable Declaration ---

#[derive(Debug)]
pub struct VarDec<'src> {
    pub names: Vec<&'src str>,
    pub ty: Type<'src>,
}

// --- Subroutine Declaration ---

#[derive(Debug)]
pub struct SubroutineDec<'src> {
    pub name: &'src str,
    pub kind: SubroutineKind,
    pub return_ty: ReturnType<'src>,
    pub parameters: Vec<Parameter<'src>>,
    pub body: SubroutineBody<'src>,
}
// --- Subroutine Types(Kind)---

#[derive(Debug, Clone, PartialEq)]
pub enum SubroutineKind {
    Constructor,
    Function,
    Method,
}

// --- Subroutine Body ---

#[derive(Debug)]
pub struct SubroutineBody<'src> {
    pub variables: Vec<VarDec<'src>>,
    pub statements: Vec<Statement<'src>>,
}

// --- Subroutine Call Declaration ---

#[derive(Debug, Clone, PartialEq)]
pub struct SubroutineCall<'src> {
    pub name: &'src str,
    pub receiver: Option<&'src str>,
    pub args: Vec<Expr<'src>>,
}

// --- Varable Type(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum Type<'src> {
    Int,
    Char,
    Boolean,
    Class(&'src str),
}

// --- Jack Return Types(Kind) ---

#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType<'src> {
    Void,
    Type(Type<'src>),
}

// --- Parameter/Argument ---

#[derive(Debug)]
pub struct Parameter<'src> {
    pub name: &'src str,
    pub ty: Type<'src>,
}

// impl fmt::Display for Class {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         let mut out = String::new();
//         self.write_xml(&mut out, 0);
//         write!(f, "{out}")
//     }
// }

// impl fmt::Display for ClassVarKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Field => write!(f, "field"),
//             Self::Static => write!(f, "static"),
//         }
//     }
// }

// impl fmt::Display for SubroutineKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Constructor => write!(f, "constructor"),
//             Self::Function => write!(f, "function"),
//             Self::Method => write!(f, "method"),
//         }
//     }
// }

// impl fmt::Display for DataKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Int => write!(f, "int"),
//             Self::Char => write!(f, "char"),
//             Self::Boolean => write!(f, "boolean"),
//             Self::Class(name) => write!(f, "{name}"),
//         }
//     }
// }

// impl fmt::Display for ReturnKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Self::Void => write!(f, "void"),
//             Self::Kind(kind) => write!(f, "{kind}"),
//         }
//     }
// }

// impl fmt::Display for Parameter {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}: {}", self.name, self.kind)
//     }
// }
