// --- Span ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[must_use]
    pub const fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

// --- Token ---

#[derive(Debug, Clone)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

impl<'src> Token<'src> {
    #[must_use]
    pub const fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub const fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }
}

// --- TokenKind ---

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenKind<'src> {
    // Literals
    IntLiteral(u32),
    StringLiteral(&'src str),

    // Identifier
    Identifier(&'src str),

    // Keywords
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

    // Punctuation
    LBrace,
    RBrace,
    LParen,
    RParen,
    LBracket,
    RBracket,
    Dot,
    Comma,
    Semicolon,

    // Operators
    Plus,
    Minus,
    Star,
    Slash,
    Ampersand,
    Pipe,
    Gt,
    Lt,
    Equal,
    Tilde,

    Eof,
}

// --- Classification ---

impl TokenKind<'_> {
    #[must_use]
    pub const fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier(_))
    }

    #[must_use]
    pub const fn is_integer(&self) -> bool {
        matches!(self, Self::IntLiteral(_))
    }

    #[must_use]
    pub const fn is_string(&self) -> bool {
        matches!(self, Self::StringLiteral(_))
    }

    #[must_use]
    pub const fn is_keyword(&self) -> bool {
        matches!(
            self,
            Self::Class
                | Self::Constructor
                | Self::Function
                | Self::Method
                | Self::Field
                | Self::Static
                | Self::Var
                | Self::Int
                | Self::Char
                | Self::Boolean
                | Self::Void
                | Self::True
                | Self::False
                | Self::Null
                | Self::This
                | Self::Let
                | Self::Do
                | Self::If
                | Self::Else
                | Self::While
                | Self::Return
        )
    }

    #[must_use]
    pub const fn is_op(&self) -> bool {
        matches!(
            self,
            Self::Plus
                | Self::Minus
                | Self::Star
                | Self::Slash
                | Self::Ampersand
                | Self::Pipe
                | Self::Gt
                | Self::Lt
                | Self::Equal
                | Self::Tilde
        )
    }
}

// --- Conversion ---

impl<'src> TokenKind<'src> {
    #[must_use]
    pub const fn from_symbol(b: u8) -> Option<Self> {
        match b {
            b'{' => Some(Self::LBrace),
            b'}' => Some(Self::RBrace),
            b'(' => Some(Self::LParen),
            b')' => Some(Self::RParen),
            b'[' => Some(Self::LBracket),
            b']' => Some(Self::RBracket),
            b'.' => Some(Self::Dot),
            b',' => Some(Self::Comma),
            b';' => Some(Self::Semicolon),
            b'+' => Some(Self::Plus),
            b'-' => Some(Self::Minus),
            b'*' => Some(Self::Star),
            b'/' => Some(Self::Slash),
            b'&' => Some(Self::Ampersand),
            b'|' => Some(Self::Pipe),
            b'>' => Some(Self::Gt),
            b'<' => Some(Self::Lt),
            b'=' => Some(Self::Equal),
            b'~' => Some(Self::Tilde),
            _ => None,
        }
    }

    #[rustfmt::skip]
    #[must_use]
    pub fn from_keyword(s: &'src str) -> Option<Self> {
        let &first = s.as_bytes().first()?;
        if !matches!(first,
            b'b' | b'c' | b'd' | b'e' | b'f' | b'i' | b'l' |
            b'm' | b'n' | b'r' | b's' | b't' | b'v' | b'w'
        ) {
            return None;
        }

        match s {
            "boolean"     => Some(Self::Boolean),
            "char"        => Some(Self::Char),
            "class"       => Some(Self::Class),
            "constructor" => Some(Self::Constructor),
            "do"          => Some(Self::Do),
            "else"        => Some(Self::Else),
            "false"       => Some(Self::False),
            "field"       => Some(Self::Field),
            "function"    => Some(Self::Function),
            "if"          => Some(Self::If),
            "int"         => Some(Self::Int),
            "let"         => Some(Self::Let),
            "method"      => Some(Self::Method),
            "null"        => Some(Self::Null),
            "return"      => Some(Self::Return),
            "static"      => Some(Self::Static),
            "this"        => Some(Self::This),
            "true"        => Some(Self::True),
            "var"         => Some(Self::Var),
            "void"        => Some(Self::Void),
            "while"       => Some(Self::While),
            _             => None,
        }
    }
}

// --- Display ---

impl std::fmt::Display for TokenKind<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IntLiteral(n)    => return write!(f, "{n}"),
            Self::StringLiteral(s) => return write!(f, "\"{s}\""),
            Self::Identifier(name) => return f.write_str(name),
            _ => {}
        }

        f.write_str(match self {
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

            Self::LBrace      => "{",
            Self::RBrace      => "}",
            Self::LParen      => "(",
            Self::RParen      => ")",
            Self::LBracket    => "[",
            Self::RBracket    => "]",
            Self::Dot         => ".",
            Self::Comma       => ",",
            Self::Semicolon   => ";",

            Self::Plus        => "+",
            Self::Minus       => "-",
            Self::Star        => "*",
            Self::Slash       => "/",
            Self::Ampersand   => "&",
            Self::Pipe        => "|",
            Self::Gt          => ">",
            Self::Lt          => "<",
            Self::Equal       => "=",
            Self::Tilde       => "~",

            Self::Eof         => "end of file",

            _ => unreachable!(),
        })
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}..{}", self.kind, self.span.start, self.span.end)
    }
}
