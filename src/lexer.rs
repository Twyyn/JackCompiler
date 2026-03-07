use std::str::FromStr;

use crate::JACK_INT_MAX;
use crate::token::{Keyword, LexerError, Span, Symbol, Token, TokenKind};

pub struct Lexer<'src> {
    source: &'src str,
    source_as_bytes: &'src [u8],
    pos: usize,
    line: u32,
    column: u16,
    tokens: Vec<Token<'src>>,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            source_as_bytes: source.as_bytes(),
            pos: 0,
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
    pub fn tokenize(mut self) -> Result<Vec<Token<'src>>, LexerError> {
        while !self.is_at_end() {
            self.scan_token()?;
        }
        self.add_token(TokenKind::Eof, self.pos);
        Ok(self.tokens)
    }

    // --- Scanner Dispatch ---

    #[rustfmt::skip]
    fn scan_token(&mut self) -> Result<(), LexerError> {
        let start = self.pos;
        let c = self.advance();

        match c {
            b'/' if self.peek() == b'*' || self.peek() == b'/' => {
                self.skip_comment();
            }
            _ if c.is_ascii_whitespace() => {
                self.advance_while(|b| b.is_ascii_whitespace());
            }

            b'"'                               => self.scan_string(start)?,
            b'0'..=b'9'                        => self.scan_integer(start)?,
            b'a'..=b'z' | b'A'..=b'Z' | b'_'  => self.scan_word(start),
            _                                  => self.scan_symbol(start)?,
        }

        Ok(())
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize) -> Result<(), LexerError> {
        let string_start = self.pos;
        self.advance_while(|b| b != b'"');
        let lexeme = self.slice(string_start, self.pos);

        if !self.is_at_end() && self.peek() == b'"' {
            self.advance();
        } else {
            return Err(LexerError::UnterminatedString);
        }

        self.add_token(TokenKind::StringConstant(lexeme), start);
        Ok(())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize) -> Result<(), LexerError> {
        self.advance_while(|b| b.is_ascii_digit());
        let lexeme = self.slice(start, self.pos);

        let value = match lexeme.parse::<u32>() {
            Ok(n) if n <= JACK_INT_MAX => n as u16,
            Ok(n) => return Err(LexerError::IntegerOutOfRange(n)),
            Err(e) => return Err(LexerError::InvalidInteger(e.to_string())),
        };

        self.add_token(TokenKind::IntegerConstant(value), start);
        Ok(())
    }

    fn scan_word(&mut self, start: usize) {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = self.slice(start, self.pos);

        let kind = match Keyword::from_str(lexeme) {
            Ok(keyword) => TokenKind::Keyword(keyword),
            Err(()) => TokenKind::Identifier(lexeme),
        };

        self.add_token(kind, start);
    }

    fn scan_symbol(&mut self, start: usize) -> Result<(), LexerError> {
        let c = self.source_as_bytes[self.pos - 1] as char;

        let kind = match Symbol::from_char(c) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol(c.to_string())),
        };

        self.add_token(kind, start);
        Ok(())
    }

    // --- Comments ---

    fn skip_comment(&mut self) {
        match self.peek() {
            // Block comment
            b'*' => {
                self.advance(); // Skip '*'
                while !self.is_at_end() {
                    if self.peek() == b'*' && self.peek_next() == b'/' {
                        self.advance(); // Skip '*'
                        self.advance(); // Skip '/'
                        break;
                    }
                    self.advance();
                }
            }
            // Inline comment
            _ => self.advance_while(|b| b != b'\n'),
        }
    }

    // --- Token Helper ---

    #[allow(clippy::cast_possible_truncation)]
    fn add_token(&mut self, kind: TokenKind<'src>, start: usize) {
        let len = if matches!(kind, TokenKind::Eof) {
            0
        } else {
            self.pos - start
        };

        let span = Span::new(
            start as u32,
            len as u16,
            self.line,
            self.column.saturating_sub(len as u16),
        );

        self.tokens.push(Token::new(kind, span));
    }

    // --- Byte Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.pos >= self.source_as_bytes.len()
    }

    fn peek(&self) -> u8 {
        if self.is_at_end() {
            b'\0'
        } else {
            self.source_as_bytes[self.pos]
        }
    }

    fn peek_next(&self) -> u8 {
        if self.pos + 1 >= self.source_as_bytes.len() {
            b'\0'
        } else {
            self.source_as_bytes[self.pos + 1]
        }
    }

    fn advance(&mut self) -> u8 {
        let current_byte = self.source_as_bytes[self.pos];
        self.pos += 1;

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
