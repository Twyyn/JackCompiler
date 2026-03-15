// --- Token ---
#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn is_eof(&self) -> bool {
        matches!(self.kind, TokenKind::Eof)
    }
}

// --- Token Kind ---
#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
    // Literals
    IntLiteral(u32),       // IntegerConstant
    StringLiteral(String), // StringConstant

    // Identifier
    Identifier(String),

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
    Semicolon, // ','

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

impl TokenKind {
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
            b'>' => Some(Self::Lt),
            b'<' => Some(Self::Gt),
            b'=' => Some(Self::Equal),
            b'~' => Some(Self::Tilde),
            _ => None,
        }
    }

    #[rustfmt::skip]
    pub fn from_keyword(s: &str) -> Option<TokenKind> {
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

// --- Impl Display ---
impl std::fmt::Display for TokenKind {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // -- Literals --
            Self::IntLiteral(_)            => write!(f, "integer constant"),
            Self::StringLiteral(_)         => write!(f, "string constant"),

            // -- Identifier --
            Self::Identifier(name) => write!(f, "'{name}'"),

            // --Keywords --
            Self::Class        => write!(f, "'class'"),
            Self::Constructor  => write!(f, "'constructor'"),
            Self::Function     => write!(f, "'function'"),
            Self::Method       => write!(f, "'method'"),
            Self::Field        => write!(f, "'field'"),
            Self::Static       => write!(f, "'static'"),
            Self::Var          => write!(f, "'var'"),
            Self::Int          => write!(f, "'int'"),
            Self::Char         => write!(f, "'char'"),
            Self::Boolean      => write!(f, "'boolean'"),
            Self::Void         => write!(f, "'void'"),
            Self::True         => write!(f, "'true'"),
            Self::False        => write!(f, "'false'"),
            Self::Null         => write!(f, "'null'"),
            Self::This         => write!(f, "'this'"),
            Self::Let          => write!(f, "'let'"),
            Self::Do           => write!(f, "'do'"),
            Self::If           => write!(f, "'if'"),
            Self::Else         => write!(f, "'else'"),
            Self::While        => write!(f, "'while'"),
            Self::Return       => write!(f, "'return'"),

            // -- Punctuation(Symbols) --
            Self::LBrace       => write!(f, "'{{'"),
            Self::RBrace       => write!(f, "'}}'"),
            Self::LParen       => write!(f, "'('"),
            Self::RParen       => write!(f, "')'"),
            Self::LBracket     => write!(f, "'['"),
            Self::RBracket     => write!(f, "']'"),
            Self::Dot          => write!(f, "'.'"),
            Self::Comma        => write!(f, "','"),
            Self::Semicolon    => write!(f, "';'"),

            // -- Operators(Symbols) --
            Self::Plus         => write!(f, "'+'"),
            Self::Minus        => write!(f, "'-'"),
            Self::Star         => write!(f, "'*'"),
            Self::Slash        => write!(f, "'/'"),
            Self::Ampersand    => write!(f, "'&'"),
            Self::Pipe         => write!(f, "'|'"),
            Self::Gt           => write!(f, "'>'"),
            Self::Lt           => write!(f, "'<'"),
            Self::Equal        => write!(f, "'='"),
            Self::Tilde        => write!(f, "'~'"),

            // -- Special --
            Self::Eof => write!(f, "'end of input'"),
        }
    }
}
