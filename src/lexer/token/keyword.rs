use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
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

impl Keyword {
    #[rustfmt::skip]
    pub fn from_slice(slice: &str) -> Option<Keyword> {
        match slice {
            "class"       => Some(Self::Class),
            "constructor" => Some(Self::Constructor),
            "function"    => Some(Self::Function),
            "method"      => Some(Self::Method),
            "field"       => Some(Self::Field),
            "static"      => Some(Self::Static),
            "var"         => Some(Self::Var),
            "int"         => Some(Self::Int),
            "char"        => Some(Self::Char),
            "boolean"     => Some(Self::Boolean),
            "void"        => Some(Self::Void),
            "true"        => Some(Self::True),
            "false"       => Some(Self::False),
            "null"        => Some(Self::Null),
            "this"        => Some(Self::This),
            "let"         => Some(Self::Let),
            "do"          => Some(Self::Do),
            "if"          => Some(Self::If),
            "else"        => Some(Self::Else),
            "while"       => Some(Self::While),
            "return"      => Some(Self::Return),
            _             => None,
        }
    }
}

impl fmt::Display for Keyword {
    #[rustfmt::skip]
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
            Self::Return      => "return",
        };

        f.write_str(s)
    }
}
