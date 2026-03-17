pub mod error;
pub mod token;

pub use error::LexerError;
pub use token::{Span, Token, TokenKind};

use crate::JACK_INT_MAX;

#[derive(Debug)]
pub struct Lexer<'src> {
    source_bytes: &'src [u8],
    cursor: usize,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source_bytes: source.as_bytes(),
            cursor: 0,
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token<'src>>, LexerError> {
        // Estimate ~1 token per 5 bytes of source (typical for C code).
        let mut tokens = Vec::with_capacity(self.source_bytes.len() / 4);
        loop {
            let token = self.next_token()?;
            let is_eof = token.is_eof();
            tokens.push(token);
            if is_eof {
                break;
            }
        }
        Ok(&'src tokens)
    }

    // --- Scanner Dispatch ---

    fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_comments_whitespace()?;

        let start = self.cursor;
        let Some(b) = self.peek() else {
            return Ok(Token::new(TokenKind::Eof, Span::new(start as u32, 0)));
        };

        match b {
            b'"' => self.scan_string(start),
            b'0'..=b'9' => self.scan_integer(start),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.scan_word(start)?),
            _ => self.scan_symbol(b, start),
        }
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize) -> Result<Token, LexerError> {
        self.bump_cursor(1)?; // consume opening quote

        loop {
            match self.source_bytes.get(self.cursor) {
                Some(b'"') => {
                    self.bump_cursor(1)?; // consume closing quote
                    let lexeme = unsafe {
                        std::str::from_utf8_unchecked(&self.source_bytes[start..self.cursor])
                    };

                    return Ok(Token::new(
                        TokenKind::StringLiteral(lexeme),
                        Span::new(start as u32, self.cursor as u32),
                    ));
                }
                Some(b'\n') | None => return Err(LexerError::UnterminatedString),
                Some(_) => self.bump_cursor(1)?,
            }
        }
    }

    fn scan_integer(&mut self, start: usize) -> Result<Token, LexerError> {
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.bump_cursor(1)?;
        }

        let max = u32::from(JACK_INT_MAX);
        let mut value: u32 = 0;

        for &d in &self.source_bytes[start..self.cursor] {
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(u32::from(d - b'0')))
                .filter(|&v| v <= max)
                .ok_or_else(|| {
                    // Error path only — reconstruct for the diagnostic
                    let raw = std::str::from_utf8(&self.source_bytes[start..self.cursor])
                        .iter()
                        .collect()
                        .parse::<u64>()
                        .unwrap_or(u64::MAX);
                    LexerError::IntegerOutOfRange(raw)
                })?;
        }

        Ok(Token::new(
            TokenKind::IntLiteral(value),
            Span::new(start as u32, self.cursor as u32),
        ))
    }

    fn scan_word(&mut self, start: usize) -> Result<Token, LexerError> {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');
        let lexeme = std::str::from_utf8(&self.source_bytes[start..self.cursor]).unwrap_or("");

        let kind = match TokenKind::from_keyword(lexeme) {
            Some(keyword) => keyword,
            None => TokenKind::Identifier(lexeme),
        };

        Ok(Token::new(
            kind,
            Span::new(start as u32, self.cursor as u32),
        ))
    }

    fn scan_symbol(&mut self, b: u8, start: usize) -> Result<Token, LexerError> {
        let kind = match TokenKind::from_symbol(b) {
            Some(symbol) => symbol,
            None => return Err(LexerError::InvalidSymbol(b.to_string())),
        };

        Ok(Token::new(
            kind,
            Span::new(start as u32, self.cursor as u32),
        ))
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
                        self.bump_cursor(2)?;
                        self.skip_line_comment();
                    }
                    Some(b'*') => {
                        self.bump_cursor(2)?;
                        self.skip_block_comment()?;
                    }
                    _ => break,
                },
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_line_comment(&mut self) -> Result<(), LexerError> {
        while self.has_more_bytes() {
            if matches!(self.peek(), Some(b'\n')) {
                break;
            }
            self.bump_cursor(1)?;
        }

        Ok(())
    }

    fn skip_block_comment(&mut self) -> Result<(), LexerError> {
        while self.has_more_bytes() {
            if matches!((self.peek(), self.peek_next()), (Some(b'*'), Some(b'/'))) {
                self.bump_cursor(1)?;
                return Ok(());
            }
            self.bump_cursor(1)?;
        }
        Err(LexerError::UnterminatedComment)
    }

    fn bump_cursor(&mut self, length: usize) -> Result<(), LexerError> {
        if self.cursor + length > self.source_bytes.len() {
            return Err(LexerError::CursorOutofBounds);
        }
        self.cursor += length;
        Ok(())
    }

    #[inline]
    fn has_more_bytes(&self) -> bool {
        self.cursor < self.source_bytes.len()
    }

    #[inline]
    fn peek(&self) -> Option<u8> {
        self.source_bytes.get(self.cursor).copied()
    }

    #[inline]
    fn peek_next(&self) -> Option<u8> {
        if self.has_more_bytes() {
            return Some(self.source_bytes[self.cursor + 1]);
        }
        None
    }

    #[inline(always)]
    fn advance_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while let Some(&b) = self.source_bytes.get(self.cursor) {
            if !predicate(b) {
                break;
            }
            self.cursor += 1;
        }
    }
}
