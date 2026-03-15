pub mod error;
pub mod token;

pub use error::LexerError;

//use std::str::FromStr;

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'src> {
    bytes: &'src [u8],
    cursor: usize,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            bytes: source.as_bytes(),
            cursor: 0,
        }
    }

    pub fn tokenize(mut self) -> Result<Vec<Token<'src>>, LexerError> {
        let mut tokens = Vec::with_capacity(self.bytes.len() / 4);
        loop {
            tokens.push(self.scan_token()?);
        }

        Ok(tokens)
    }

    // --- Scanner Dispatch ---

    fn scan_token(&mut self) -> Result<Token<'src>, LexerError> {
        self.skip_comments_whitespace()?;

        let start = self.cursor;
        let Some(b) = self.advance() else {
            return Ok(Token::new(TokenKind::Eof, Span::new(start, 0)));
        };

        match b {
            b'"' => self.scan_string(start),
            b'0'..=b'9' => self.scan_integer(start),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.scan_word(start)),
            _ => self.scan_symbol(b, start),
        }
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize) -> Result<Token<'src>, LexerError> {
        let content_start = self.cursor;

        while let Some(&b) = self.bytes.get(self.cursor) {
            match b {
                b'"' => {
                    let lexeme = self.slice(content_start, self.cursor);
                    self.cursor += 1; // consume closing quote

                    return Ok(Token::new(
                        TokenKind::StringConstant(lexeme),
                        Span::new(start, self.cursor - start),
                    ));
                }
                b'\n' => return Err(LexerError::UnterminatedString),
                _ => self.cursor += 1,
            }
        }
        Err(LexerError::UnterminatedString)
    }
    fn scan_integer(&mut self, start: usize) -> Result<Token<'src>, LexerError> {
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.cursor += 1;
        }

        let max = u32::from(JACK_INT_MAX);
        let mut value: u32 = 0;

        for &d in &self.bytes[start..self.cursor] {
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(u32::from(d - b'0')))
                .filter(|&v| v <= max)
                .ok_or_else(|| {
                    // Error path only — reconstruct for the diagnostic
                    let raw = self
                        .slice(start, self.cursor)
                        .parse::<u64>()
                        .unwrap_or(u64::MAX);
                    LexerError::IntegerOutOfRange(raw)
                })?;
        }

        Ok(Token::new(
            TokenKind::IntegerConstant(value),
            Span::new(start, self.cursor - start),
        ))
    }

    fn scan_word(&mut self, start: usize) -> Token<'src> {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = self.slice(start, self.cursor);

        let kind = match Keyword::from_slice(lexeme) {
            Some(kw) => TokenKind::Keyword(kw),
            None => TokenKind::Identifier(lexeme),
        };

        Token::new(kind, Span::new(start, self.cursor - start))
    }

    fn scan_symbol(&mut self, b: u8, start: usize) -> Result<Token<'src>, LexerError> {
        let kind = match Symbol::from_byte(b) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol(b.to_string())),
        };

        Ok(Token::new(kind, Span::new(start, self.cursor - start)))
    }

    // --- Skip Comment & Whitespace ---

    fn skip_comments_whitespace(&mut self) -> Result<(), LexerError> {
        loop {
            match self.peek() {
                Some(b) if b.is_ascii_whitespace() => {
                    self.advance_while(|b| b.is_ascii_whitespace());
                }
                Some(b'/') => match self.peek_next() {
                    Some(b'/') => {
                        self.cursor += 2;
                        self.skip_line_comment();
                    }
                    Some(b'*') => {
                        self.cursor += 2;
                        self.skip_block_comment()?;
                    }
                    _ => break,
                },
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_line_comment(&mut self) {
        while let Some(b) = self.peek() {
            if b == b'\n' {
                break;
            }
            self.cursor += 1;
        }
    }

    fn skip_block_comment(&mut self) -> Result<(), LexerError> {
        let bytes = self.bytes;
        while self.cursor + 1 < bytes.len() {
            if bytes[self.cursor] == b'*' && bytes[self.cursor + 1] == b'/' {
                self.cursor += 2;
                return Ok(());
            }
            self.cursor += 1;
        }
        Err(LexerError::UnterminatedComment)
    }
    // --- Char Navigation Helpers ---

    #[inline(always)]
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    #[inline(always)]
    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.cursor + 1).copied()
    }

    #[inline(always)]
    fn advance(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.cursor += 1;
        Some(b)
    }

    #[inline(always)]
    fn advance_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while let Some(&b) = self.bytes.get(self.cursor) {
            if !predicate(b) {
                break;
            }
            self.cursor += 1;
        }
    }

    #[inline(always)]
    fn slice(&self, start: usize, end: usize) -> &'src str {
        debug_assert!(start <= end && end <= self.bytes.len());
        unsafe {
            let bytes = self.bytes.get_unchecked(start..end);
            core::str::from_utf8_unchecked(bytes)
        }
    }
}
