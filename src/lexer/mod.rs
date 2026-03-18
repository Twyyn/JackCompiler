pub mod error;
pub mod token;

pub use error::LexerError;
pub use token::{Span, Token, TokenKind};

use crate::JACK_INT_MAX;

#[derive(Debug)]
pub struct Lexer<'src> {
    source_bytes: &'src [u8],
    cursor: usize,
    is_at_end: bool,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source_bytes: source.as_bytes(),
            cursor: 0,
            is_at_end: false,
        }
    }

    #[allow(clippy::missing_errors_doc)]
    pub fn tokenize(self) -> Result<Vec<Token<'src>>, LexerError> {
        self.collect()
    }
}
// ── Scanner Dispatch ─────────────────────────────────────────────

#[allow(clippy::cast_possible_truncation)]
impl<'src> Lexer<'src> {
    fn next_token(&mut self) -> Result<Token<'src>, LexerError> {
        self.skip_comments_whitespace()?;

        let start = self.cursor;
        let Some(b) = self.peek() else {
            return Ok(Token::new(TokenKind::Eof, Span::new(start as u32, 0)));
        };

        match b {
            b'"' => self.scan_string(start),
            b'0'..=b'9' => self.scan_integer(start),
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => Ok(self.scan_word(start)),
            _ => self.scan_symbol(b, start),
        }
    }
}

// ── Scanners ─────────────────────────────────────────────

#[allow(clippy::cast_possible_truncation)]
impl<'src> Lexer<'src> {
    fn scan_integer(&mut self, start: usize) -> Result<Token<'src>, LexerError> {
        while self.peek().is_some_and(|b| b.is_ascii_digit()) {
            self.bump_cursor(1)?;
        }

        let max = JACK_INT_MAX;
        let mut value: u32 = 0;

        for &d in &self.source_bytes[start..self.cursor] {
            value = value
                .checked_mul(10)
                .and_then(|v| v.checked_add(u32::from(d - b'0')))
                .filter(|&v| v <= max)
                .ok_or_else(|| {
                    let value = self
                        .lexeme_from_slice(start, self.cursor)
                        .parse::<u64>()
                        .unwrap_or(u64::MAX);
                    LexerError::IntegerOutOfRange(value)
                })?;
        }

        Ok(Token::new(
            TokenKind::IntLiteral(value),
            Span::new(start as u32, self.cursor as u32),
        ))
    }

    fn scan_string(&mut self, start: usize) -> Result<Token<'src>, LexerError> {
        self.bump_cursor(1)?;

        loop {
            match self.source_bytes.get(self.cursor) {
                Some(b'"') => {
                    self.bump_cursor(1)?;

                    let lexeme = self.lexeme_from_slice(start + 1, self.cursor - 1);
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

    fn scan_symbol(&mut self, b: u8, start: usize) -> Result<Token<'src>, LexerError> {
        let Some(kind) = TokenKind::from_symbol(b) else {
            return Err(LexerError::InvalidSymbol(b.to_string()));
        };

        self.bump_cursor(1)?;
        Ok(Token::new(
            kind,
            Span::new(start as u32, self.cursor as u32),
        ))
    }

    fn scan_word(&mut self, start: usize) -> Token<'src> {
        self.advance_while(|b| b.is_ascii_alphanumeric() || b == b'_');

        let lexeme = self.lexeme_from_slice(start, self.cursor);

        let kind = match TokenKind::from_keyword(lexeme) {
            Some(keyword) => keyword,
            None => TokenKind::Identifier(lexeme),
        };

        Token::new(kind, Span::new(start as u32, self.cursor as u32))
    }
}

// ── Whitespace & Comments ─────────────────────────────────────────────

impl Lexer<'_> {
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

    fn skip_line_comment(&mut self) {
        self.advance_while(|b| b != b'\n');
    }

    fn skip_block_comment(&mut self) -> Result<(), LexerError> {
        while self.has_more_bytes() {
            if matches!((self.peek(), self.peek_next()), (Some(b'*'), Some(b'/'))) {
                self.bump_cursor(2)?;
                return Ok(());
            }
            self.bump_cursor(1)?;
        }
        Err(LexerError::UnterminatedComment)
    }
}

// ── Utility ─────────────────────────────────────────────

impl<'src> Lexer<'src> {
    fn lexeme_from_slice(&self, start: usize, end: usize) -> &'src str {
        unsafe { std::str::from_utf8_unchecked(&self.source_bytes[start..end]) }
    }
}

// ── Cursor Primitives ─────────────────────────────────────────────

impl Lexer<'_> {
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
        self.source_bytes.get(self.cursor + 1).copied()
    }

    #[inline]
    fn advance_while(&mut self, predicate: impl Fn(u8) -> bool) {
        while self
            .source_bytes
            .get(self.cursor)
            .is_some_and(|&b| predicate(b))
        {
            self.cursor += 1;
        }
    }
}

// ── Iterator ─────────────────────────────────────────────

impl<'src> Iterator for Lexer<'src> {
    type Item = Result<Token<'src>, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.is_at_end {
            return None;
        }

        match self.next_token() {
            Ok(token) => {
                if token.is_eof() {
                    self.is_at_end = true;
                }
                Some(Ok(token))
            }
            Err(e) => {
                self.is_at_end = true;
                Some(Err(e))
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        if self.is_at_end {
            return (0, Some(0));
        }
        let remaining = self.source_bytes.len().saturating_sub(self.cursor);
        (1, Some(remaining + 1))
    }
}
