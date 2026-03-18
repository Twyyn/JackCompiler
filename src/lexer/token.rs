use crate::parser::ast::BinaryOp;

// --- Token ---
#[derive(Debug)]
pub struct Token<'src> {
    pub kind: TokenKind<'src>,
    pub span: Span,
}

impl<'src> Token<'src> {
    #[must_use]
    pub fn new(kind: TokenKind<'src>, span: Span) -> Self {
        Self { kind, span }
    }

    #[must_use]
    pub fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }
}

// --- Token Kind ---
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind<'src> {
    // Literals
    IntLiteral(u32),          // IntegerConstant
    StringLiteral(&'src str), // StringConstant

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

    // Punctuation(Symbols)
    LBrace,    // '{'
    RBrace,    // '}'
    LParen,    // '('
    RParen,    // ')'
    LBracket,  // '['
    RBracket,  // ']'
    Dot,       // '.'
    Comma,     // ','
    Semicolon, // ';'

    // Operators(Symbols)
    Plus,      // '+'
    Minus,     // '-'
    Star,      // '*'
    Slash,     // '/'
    Ampersand, // '&'
    Pipe,      // '|'
    Gt,        // '>'
    Lt,        // '<'
    Equal,     // '='
    Tilde,     // '~'

    // Special
    Eof,
}

impl<'src> TokenKind<'src> {
    #[must_use]
    pub fn from_symbol(b: u8) -> Option<Self> {
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
    pub fn from_keyword(s: &'src str) -> Option<TokenKind<'src>> {
        let first_char = s.as_bytes()[0];
        if !matches!(first_char,
            b'c' | b'f' | b'm' | b's' | b'v' | b'e' | b'i' | b'b' |
            b't' | b'n' | b'r' | b'l' | b'd' | b'w'
        ) {
            return None;
        }

        match s {
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

impl TokenKind<'_> {
    #[must_use]
    pub fn as_binary_op(&self) -> Option<BinaryOp> {
        match self {
            Self::Plus => Some(BinaryOp::Add),
            Self::Minus => Some(BinaryOp::Sub),
            Self::Star => Some(BinaryOp::Mul),
            Self::Slash => Some(BinaryOp::Div),
            Self::Ampersand => Some(BinaryOp::And),
            Self::Pipe => Some(BinaryOp::Or),
            Self::Lt => Some(BinaryOp::Lt),
            Self::Gt => Some(BinaryOp::Gt),
            Self::Equal => Some(BinaryOp::Eq),
            _ => None,
        }
    }
}


// --- Span ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: u32,
    pub end: u32,
}

impl Span {
    #[must_use]
    pub fn new(start: u32, end: u32) -> Self {
        Self { start, end }
    }
}

// --- Impl Displays ---
impl std::fmt::Display for TokenKind<'_> {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            // -- Literals --
            Self::IntLiteral(_)            =>  "integer constant",
            Self::StringLiteral(_)         =>  "string constant",

            // -- Identifier --
            Self::Identifier(name) =>  name,

            // --Keywords --
            Self::Class        => "class",
            Self::Constructor  => "constructor",
            Self::Function     => "function",
            Self::Method       => "method",
            Self::Field        => "field",
            Self::Static       =>  "static",
            Self::Var          =>  "var",
            Self::Int          =>  "int",
            Self::Char         =>  "char",
            Self::Boolean      =>  "boolean",
            Self::Void         =>  "void",
            Self::True         =>  "true",
            Self::False        =>  "false",
            Self::Null         =>  "null",
            Self::This         =>  "this",
            Self::Let          =>  "let",
            Self::Do           =>  "do",
            Self::If           =>  "if",
            Self::Else         =>  "else",
            Self::While        =>  "while",
            Self::Return       =>  "return",

            // -- Punctuation(Symbols) --
            Self::LBrace       =>  "{",
            Self::RBrace       =>  "}",
            Self::LParen       =>  "(",
            Self::RParen       =>  ")",
            Self::LBracket     =>  "[",
            Self::RBracket     =>  "]",
            Self::Dot          =>  ".",
            Self::Comma        =>  ",",
            Self::Semicolon    =>  ";",

            // -- Operators(Symbols) --
            Self::Plus         =>  "+",
            Self::Minus        =>  "-",
            Self::Star         =>  "*",
            Self::Slash        =>  "/",
            Self::Ampersand    =>  "&",
            Self::Pipe         =>  "|",
            Self::Gt           =>  ">",
            Self::Lt           =>  "<",
            Self::Equal        =>  "=",
            Self::Tilde        =>  "~",

            // -- Special --
            Self::Eof =>  "end of file",
        };
        f.write_str(s)
    }
}

impl std::fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} at {}..{}", self.kind, self.span.start, self.span.end)
    }
}
