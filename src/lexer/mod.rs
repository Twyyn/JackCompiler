pub mod error;
pub mod token;

pub use error::LexerError;

use std::str::FromStr;

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenType};

pub struct Lexer<'src> {
    source: &'src str,
    source_as_bytes: &'src [u8],
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
            source_as_bytes: source.as_bytes(),
            position: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
        }
    }

    /// Convert the source text into a sequence of lexical tokens.
    ///
    /// The lexer scans the input byte-by-byte and emits `Token` instances.
    ///
    /// # Errors
    ///
    /// A `TokenError` is returned when the scanner encounters invalid input,
    /// such as an invalid symbol, unterminated string literal, integer that
    /// cannot be parsed or is out of the allowed range, or any other
    /// malformed token.
    pub fn tokenize(mut self) -> Result<Vec<Token>, LexerError> {
        while !self.is_at_end() {
            self.scan_token()?;
        }
        self.add_token(TokenType::Eof, self.position, self.column);

        Ok(self.tokens)
    }

    // --- Scanner Dispatch ---

    fn scan_token(&mut self) -> Result<(), LexerError> {
        let start = self.position;
        let column = self.column;
        let c = self.advance();

        match c {
            b'/' if self.peek() == b'*' || self.peek() == b'/' => {
                self.skip_comment()?;
            }
            _ if c.is_ascii_whitespace() => {
                self.advance_while(|b| b.is_ascii_whitespace());
            }

            b'"' => self.scan_string(start, column)?,
            b'0'..=b'9' => self.scan_integer(start, column)?,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => self.scan_word(start, column),
            _ => self.scan_symbol(c, start, column)?,
        }

        Ok(())
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize, column: u16) -> Result<(), LexerError> {
        let string_start = self.position;
        self.advance_while(|b| b != b'"');
        let lexeme = self.slice(string_start, self.position);

        if !self.is_at_end() && self.peek() == b'"' {
            self.advance();
        } else {
            return Err(LexerError::UnterminatedString);
        }

        self.add_token(TokenType::StringConstant(lexeme.into()), start, column);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize, column: u16) -> Result<(), LexerError> {
        self.advance_while(|b| b.is_ascii_digit());
        let lexeme = self.slice(start, self.position);

        let value = match lexeme.parse::<u32>() {
            Ok(n) if n <= JACK_INT_MAX => n as u16,
            Ok(n) => return Err(LexerError::IntegerOutOfRange(n)),
            Err(e) => return Err(LexerError::InvalidInteger(e.to_string())),
        };

        self.add_token(TokenType::IntegerConstant(value), start, column);
        Ok(())
    }

    fn scan_word(&mut self, start: usize, column: u16) {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = self.slice(start, self.position);

        let token_type = match Keyword::from_str(lexeme) {
            Ok(keyword) => TokenType::Keyword(keyword),
            Err(()) => TokenType::Identifier(lexeme.into()),
        };

        self.add_token(token_type, start, column);
    }

    fn scan_symbol(&mut self, c: u8, start: usize, column: u16) -> Result<(), LexerError> {
        let token_type = match Symbol::from_char(c as char) {
            Some(symbol) => TokenType::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol((c as char).to_string())),
        };

        self.add_token(token_type, start, column);
        Ok(())
    }

    // --- Comments ---

    fn skip_comment(&mut self) -> Result<(), LexerError> {
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
            self.advance_while(|b| b != b'\n');
            Ok(())
        }
    }
    // --- Token Helper ---

    #[allow(clippy::cast_possible_truncation)]
    fn add_token(&mut self, token_type: TokenType, start: usize, column: u16) {
        let len = if matches!(token_type, TokenType::Eof) {
            0
        } else {
            self.position - start
        };

        self.tokens.push(Token::new(
            token_type,
            Span::new(start as u32, len as u16, self.line, column),
        ));
    }

    // --- Byte Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.position >= self.source_as_bytes.len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source_as_bytes[self.position]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.position + 1 >= self.source_as_bytes.len() {
            b'\0'
        } else {
            self.source_as_bytes[self.position + 1]
        }
    }

    fn advance(&mut self) -> u8 {
        let current_byte = self.source_as_bytes[self.position];
        self.position += 1;

        if current_byte == b'\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        current_byte
    }

    fn advance_while(&mut self, predicate: fn(u8) -> bool) {
        while !self.is_at_end() && predicate(self.peek()) {
            self.advance();
        }
    }

    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }
}
