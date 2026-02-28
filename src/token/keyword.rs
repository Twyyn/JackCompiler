use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Keyword {
    Class,
    Constructor,
    Function,
    Method,
    Field,
    Static,
    Var,
    Int,
    Char,
    Boolean,
    Void,
    True,
    False,
    Null,
    This,
    Let,
    Do,
    If,
    Else,
    While,
    Return,
}

impl FromStr for Keyword {
    type Err = ();

    #[rustfmt::skip]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "class"       => Ok(Self::Class),
            "constructor" => Ok(Self::Constructor),
            "function"    => Ok(Self::Function),
            "method"      => Ok(Self::Method),
            "field"       => Ok(Self::Field),
            "static"      => Ok(Self::Static),
            "var"         => Ok(Self::Var),
            "int"         => Ok(Self::Int),
            "char"        => Ok(Self::Char),
            "boolean"     => Ok(Self::Boolean),
            "void"        => Ok(Self::Void),
            "true"        => Ok(Self::True),
            "false"       => Ok(Self::False),
            "null"        => Ok(Self::Null),
            "this"        => Ok(Self::This),
            "let"         => Ok(Self::Let),
            "do"          => Ok(Self::Do),
            "if"          => Ok(Self::If),
            "else"        => Ok(Self::Else),
            "while"       => Ok(Self::While),
            "return"      => Ok(Self::Return),
            _             => Err(()),
        }
    }
}

#[rustfmt::skip]
impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Class       => "class",
            Self::Constructor => "constructor",
            Self::Function    => "function",
            Self::Method      => "method",
            Self::Field       => "field",
            Self::Static      => "static",
            Self::Var         => "var",
            Self::Int         => "int",
            Self::Char        => "char",
            Self::Boolean     => "boolean",
            Self::Void        => "void",
            Self::True        => "true",
            Self::False       => "false",
            Self::Null        => "null",
            Self::This        => "this",
            Self::Let         => "let",
            Self::Do          => "do",
            Self::If          => "if",
            Self::Else        => "else",
            Self::While       => "while",
            Self::Return      => "return"
        };

        write!(f, "{s}")
    }
}
