pub mod error;
pub mod token;

pub use error::LexerError;

use std::num::ParseIntError;
use std::str::FromStr;

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenKind};

// ── Lexer Result ────────────────────────────────────────
type LexerResult<T> = std::result::Result<T, LexerError>;

pub struct Lexer<'src> {
    source: &'src str,
    bytes: &'src [u8],
    position: usize,
    line: u32,
    column: u16,
    tokens: Vec<Token>,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            bytes: source.as_bytes(),
            position: 0,
            line: 1,
            column: 1,
            tokens: Vec::with_capacity(source.len() / 2),
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
        while !self.is_at_end() {
            self.scan_token()?;
        }
        self.add_token(TokenKind::Eof, self.position, self.column);

        Ok(self.tokens)
    }

    // --- Scanner Dispatch ---

    fn scan_token(&mut self) -> LexerResult<()> {
        let start = self.position;
        let column = self.column;
        let c = self.advance();

        match c {
            b'/' if self.peek() == b'*' || self.peek() == b'/' => {
                self.skip_comment()?;
            }
            b if b.is_ascii_whitespace() => {
                self.advance_while(u8::is_ascii_whitespace);
            }

            b'"' => self.scan_string(start, column)?,
            b'0'..=b'9' => self.scan_integer(start, column)?,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.scan_word(start, column),
            _ => self.scan_symbol(c, start, column)?,
        }

        Ok(())
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize, column: u16) -> LexerResult<()> {
        let string_start = self.position;
        self.advance_while(|b| *b != b'"');
        let lexeme = self.slice(string_start, self.position);

        while !self.is_at_end() && self.peek() != b'"' {
            if self.peek() == b'\n' {
                return Err(LexerError::UnterminatedString);
            }
            self.advance();
        }

        self.add_token(TokenKind::StringConstant(lexeme.into()), start, column);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize, column: u16) -> LexerResult<()> {
        self.advance_while(u8::is_ascii_digit);
        let lexeme = self.slice(start, self.position);

        let value: u32 = lexeme
            .parse()
            .map_err(|e: ParseIntError| LexerError::InvalidInteger(e.to_string()))?;

        if value > JACK_INT_MAX {
            return Err(LexerError::IntegerOutOfRange(value));
        }
        self.add_token(TokenKind::IntegerConstant(value as u16), start, column);
        Ok(())
    }

    fn scan_word(&mut self, start: usize, column: u16) {
        self.advance_while(|b| b.is_ascii_alphanumeric() || *b == b'_');
        let lexeme = self.slice(start, self.position);

        let kind = match Keyword::from_str(lexeme) {
            Ok(keyword) => TokenKind::Keyword(keyword),
            Err(()) => TokenKind::Identifier(lexeme.into()),
        };

        self.add_token(kind, start, column);
    }

    fn scan_symbol(&mut self, c: u8, start: usize, column: u16) -> LexerResult<()> {
        let kind = match Symbol::from_char(c as char) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol((c as char).to_string())),
        };

        self.add_token(kind, start, column);
        Ok(())
    }

    // --- Comments ---

    fn skip_comment(&mut self) -> LexerResult<()> {
        if self.peek() == b'*' {
            self.advance(); // Skip '*'
            while !self.is_at_end() {
                if self.peek() == b'*' && self.peek_next() == b'/' {
                    self.advance(); // Skip '*'
                    self.advance(); // Skip '/'
                    return Ok(());
                }
                self.advance();
            }

            Err(LexerError::UnterminatedComment)
        } else {
            self.advance_while(|b| *b != b'\n');
            Ok(())
        }
    }
    // --- Token Helper ---

    #[allow(clippy::cast_possible_truncation)]
    fn add_token(&mut self, kind: TokenKind, start: usize, column: u16) {
        let len = if matches!(kind, TokenKind::Eof) {
            0
        } else {
            self.position - start
        };

        self.tokens.push(Token::new(
            kind,
            Span::new(start as u32, len as u16, self.line, column),
        ));
    }

    // --- Byte Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.position >= self.source.len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.bytes[self.position]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.position + 1 >= self.source.len() {
            b'\0'
        } else {
            self.bytes[self.position + 1]
        }
    }

    fn advance(&mut self) -> u8 {
        let current_byte = self.bytes[self.position];
        self.position += 1;

        if current_byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        current_byte
    }

    fn advance_while<F>(&mut self, predicate: F)
    where
        F: Fn(&u8) -> bool,
    {
        while !self.is_at_end() && predicate(&self.peek()) {
            self.advance();
        }
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }
}
