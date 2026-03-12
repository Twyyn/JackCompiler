pub mod error;
pub mod token;

pub use error::LexerError;

use std::num::ParseIntError;
use std::str::{Chars, FromStr};

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenKind};

// ── Lexer Result ────────────────────────────────────────
type LexerResult<T> = std::result::Result<T, LexerError>;

pub struct Cursor<'src> {
    chars: Chars<'src>,
}

impl<'src> Cursor<'src> {
    pub fn new(input: &'src str) -> Self {
        Self {
            chars: input.chars(),
        }
    }
}

pub struct Lexer<'src> {
    source: &'src str,
    cursor: Cursor<'src>,

}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        let mut chars_iter = source.chars();
        let current = chars_iter.next();
        let next = chars_iter.next();

        Self {
            source,
            chars_iter,
            pos: 0,
            current,
            next,
        }
    }

    /// Convert the source text into a sequence of lexical tokens.
    ///
    /// The lexer scans the input byte-by-byte and emits `Token` instances.
    ///
    /// # Errors
    ///
    /// A `LexerError` is returned when the scanner encounters invalid input,
    /// such as an invalid symbol, unterminated string literal, integer that
    /// cannot be parsed or is out of the allowed range, or any other
    /// malformed token.
    pub fn tokenize(mut self) -> LexerResult<Vec<Token>> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            tokens.push(self.scan_token()?);
        }
        Ok(tokens)
    }

    // --- Scanner Dispatch ---

    fn scan_token(&mut self) -> LexerResult<Token> {
        while !self.is_at_end() {
            let start = self.pos;
            let ch = self.peek();

            match ch {
                // --- Skip whitespace ---
                c if c.is_whitespace() => {
                    self.advance(); // consume first whitespace
                    self.advance_while(char::is_whitespace); // consume the rest
                }

                // --- Skip comments ---
                '/' if self.peek_next() == '*' || self.peek_next() == '/' => {
                    self.advance(); // consume '/'
                    self.skip_comment()?;
                }

                // --- Strings ---
                '"' => {
                    self.advance(); // consume opening "
                    return self.scan_string(start);
                }

                // --- Integers ---
                '0'..='9' => {
                    self.advance(); // consume first digit
                    return self.scan_integer(start);
                }

                // --- Words / Identifiers ---
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.advance(); // consume first char
                    return self.scan_word(start);
                }

                // --- Symbols ---
                _ => {
                    self.advance(); // consume symbol
                    return self.scan_symbol(ch, start);
                }
            }
        }

        Ok(Token::new(TokenKind::Eof, Span::new(self.pos, self.pos)))
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize) -> LexerResult<Token> {
        let string_start = self.pos;

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\n' {
                return Err(LexerError::UnterminatedString);
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexerError::UnterminatedString);
        }

        let lexeme = self.slice(string_start, self.pos);
        self.advance(); // consume the closing '"'

        let kind = TokenKind::StringConstant(lexeme.into());
        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize) -> LexerResult<Token> {
        self.advance_while(|c| c.is_ascii_digit());
        let lexeme = self.slice(start, self.pos);

        let value: u64 = lexeme
            .parse()
            .map_err(|e: ParseIntError| LexerError::InvalidInteger(e.to_string()))?;

        if value > u64::from(JACK_INT_MAX) {
            return Err(LexerError::IntegerOutOfRange(value));
        }

        let kind = TokenKind::IntegerConstant(value as u32);

        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn scan_word(&mut self, start: usize) -> LexerResult<Token> {
        self.advance_while(|c| c.is_alphanumeric() || c == '_');
        let lexeme = self.slice(start, self.pos);

        let kind = if let Ok(keyword) = Keyword::from_str(lexeme) {
            TokenKind::Keyword(keyword)
        } else {
            TokenKind::Identifier(lexeme.into())
        };

        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    fn scan_symbol(&mut self, ch: char, start: usize) -> LexerResult<Token> {
        let kind = match Symbol::from_char(ch) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol(ch.to_string())),
        };

        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    // --- Comments ---

    fn skip_comment(&mut self) -> LexerResult<()> {
        if self.peek() == '*' {
            self.advance(); // skip '*'

            while !self.is_at_end() {
                if self.peek() == '*' && self.peek_next() == '/' {
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    return Ok(());
                }
                self.advance();
            }

            Err(LexerError::UnterminatedComment)
        } else {
            self.advance_while(|c| c != '\n');
            Ok(())
        }
    }

    // --- Byte Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.pos == self.source.len()
    }

    fn peek(&self) -> char {
        self.current.unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.next.unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let ch = self.current.unwrap_or('\0');
        self.pos += ch.len_utf8();

        self.current = self.next;
        self.next = self.chars_iter.next();

        ch
    }

    fn advance_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while predicate(self.peek()) {
            self.advance();
        }
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }
}
