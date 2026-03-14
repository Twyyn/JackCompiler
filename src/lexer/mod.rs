pub mod error;
pub mod token;

pub use error::LexerError;

//use std::str::FromStr;

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenKind};

#[derive(Debug)]
pub struct Lexer<'src> {
    bytes: &'src [u8],
    pos: usize,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            bytes: source.as_bytes(),
            pos: 0,
        }
    }

    // --- Scanner Dispatch ---

    fn scan_token(&mut self) -> Result<Token<'src>, LexerError> {
        self.skip_comments_whitespace()?;

        let start = self.pos;
        let Some(b) = self.advance() else {
            return Ok(Token::new(TokenKind::Eof, Span::new(start, self.pos)));
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
        let string_start = self.pos;

        while self.peek().is_some_and(|b| b != b'"' && b != b'\n') {
            self.advance();
        }

        match self.peek() {
            Some(b'\n') | None => return Err(LexerError::UnterminatedString),
            Some(b'"') => self.advance(), // consume closing '"'
            _ => unreachable!(),
        };

        let lexeme = self.slice(string_start, self.pos - 1);
        Ok(Token::new(
            TokenKind::StringConstant(lexeme.into()),
            Span::new(start, self.pos - start),
        ))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize) -> Result<Token<'src>, LexerError> {
        self.advance_while(|b| b.is_ascii_digit());
        let lexeme = self.slice(start, self.pos);

        let value: u64 = lexeme.parse().map_err(LexerError::from)?;

        if value > u64::from(JACK_INT_MAX) {
            return Err(LexerError::IntegerOutOfRange(value));
        }

        let kind = TokenKind::IntegerConstant(value as u32);

        Ok(Token::new(kind, Span::new(start, self.pos - start)))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn scan_word(&mut self, start: usize) -> Token<'src> {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = self.slice(start, self.pos);

        let kind = match Keyword::from_slice(lexeme) {
            Some(kw) => TokenKind::Keyword(kw),
            None => TokenKind::Identifier(lexeme),
        };

        Token::new(kind, Span::new(start, self.pos - start))
    }

    fn scan_symbol(&mut self, b: u8, start: usize) -> Result<Token<'src>, LexerError> {
        let kind = match Symbol::from_byte(b) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol(b.to_string())),
        };

        Ok(Token::new(kind, Span::new(start, self.pos - start)))
    }

    // --- Skip Comment & Whitespace ---

    fn skip_comments_whitespace(&mut self) -> Result<(), LexerError> {
        while let Some(b) = self.peek() {
            match b {
                b if b.is_ascii_whitespace() => {
                    self.advance_while(|b| b.is_ascii_whitespace());
                }
                b'/' if self.peek_next() == Some(b'*') || self.peek_next() == Some(b'/') => {
                    self.advance();
                    self.skip_comment()?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_comment(&mut self) -> Result<(), LexerError> {
        if self.peek() == Some(b'*') {
            self.advance(); // skip '*'

            while !self.is_at_end() {
                if self.peek() == Some(b'*') && self.peek_next() == Some(b'/') {
                    self.advance(); // skip '*'
                    self.advance(); // skip '/'
                    return Ok(());
                }
                self.advance();
            }

            Err(LexerError::UnterminatedComment)
        } else {
            self.advance_while(|c| c != b'\n');
            Ok(())
        }
    }

    // --- Char Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.pos).copied()
    }

    fn peek_next(&self) -> Option<u8> {
        self.bytes.get(self.pos + 1).copied()
    }

    fn advance(&mut self) -> Option<u8> {
        let b = self.peek()?;
        self.pos += 1;
        Some(b)
    }

    fn advance_while<F>(&mut self, predicate: F)
    where
        F: Fn(u8) -> bool,
    {
        while self.peek().is_some_and(&predicate) {
            self.advance();
        }
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'src str {
        unsafe { str::from_utf8_unchecked(&self.bytes[start..end]) }
    }
}
