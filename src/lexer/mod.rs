pub mod error;
pub mod token;

pub use error::LexerError;

use std::str::{Chars, FromStr};

use crate::JACK_INT_MAX;
use crate::lexer::token::{Keyword, Span, Symbol, Token, TokenKind};

// --- Lexer Result ---

pub type LexerResult<T> = std::result::Result<T, LexerError>;

#[derive(Debug)]
pub struct Lexer<'src> {
    source: &'src str,
    chars: Chars<'src>,
    pos: usize,
}

impl<'src> Lexer<'src> {
    #[must_use]
    pub fn new(source: &'src str) -> Self {
        Self {
            source,
            chars: source.chars(),
            pos: 0,
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
        self.skip_comments_whitespace()?;

        let start = self.pos;
        let Some(c) = self.advance() else {
            return Ok(Token::new(TokenKind::Eof, Span::new(start, self.pos)));
        };

        match c {
            '"' => self.scan_string(start),
            '0'..='9' => self.scan_integer(start),
            'a'..='z' | 'A'..='Z' | '_' => Ok(self.scan_word(start)),
            _ => self.scan_symbol(c, start),
        }
    }

    // --- Scanner Helpers ---

    fn scan_string(&mut self, start: usize) -> LexerResult<Token> {
        let string_start = self.pos;

        while self.peek().is_some_and(|c| c != '"' && c != '\n') {
            self.advance();
        }

        match self.peek() {
            Some('\n') | None => return Err(LexerError::UnterminatedString),
            Some('"') => self.advance(), // consume closing '"'
            _ => unreachable!(),
        };

        let lexeme = self.slice(string_start, self.pos - 1);
        Ok(Token::new(
            TokenKind::StringConstant(lexeme.into()),
            Span::new(start, self.pos),
        ))
    }

    #[allow(clippy::cast_possible_truncation)]
    fn scan_integer(&mut self, start: usize) -> LexerResult<Token> {
        self.advance_while(|c| c.is_ascii_digit());
        let lexeme = self.slice(start, self.pos);

        let value: u64 = lexeme.parse().map_err(LexerError::from)?;

        if value > u64::from(JACK_INT_MAX) {
            return Err(LexerError::IntegerOutOfRange(value));
        }

        let kind = TokenKind::IntegerConstant(value as u32);

        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    #[allow(clippy::unnecessary_wraps)]
    fn scan_word(&mut self, start: usize) -> Token {
        self.advance_while(|c| c.is_alphanumeric() || c == '_');
        let lexeme = self.slice(start, self.pos);

        let kind = if let Ok(keyword) = Keyword::from_str(lexeme) {
            TokenKind::Keyword(keyword)
        } else {
            TokenKind::Identifier(lexeme.into())
        };

        Token::new(kind, Span::new(start, self.pos))
    }

    fn scan_symbol(&mut self, ch: char, start: usize) -> LexerResult<Token> {
        let kind = match Symbol::from_char(ch) {
            Some(symbol) => TokenKind::Symbol(symbol),
            None => return Err(LexerError::InvalidSymbol(ch.to_string())),
        };

        Ok(Token::new(kind, Span::new(start, self.pos)))
    }

    // --- Skip Comment & Whitespace ---

    fn skip_comments_whitespace(&mut self) -> LexerResult<()> {
        while let Some(c) = self.peek() {
            match c {
                c if c.is_whitespace() => {
                    self.advance_while(char::is_whitespace);
                }
                '/' if self.peek_next() == Some('*') || self.peek_next() == Some('/') => {
                    self.advance();
                    self.skip_comment()?;
                }
                _ => break,
            }
        }
        Ok(())
    }

    fn skip_comment(&mut self) -> LexerResult<()> {
        if self.peek() == Some('*') {
            self.advance(); // skip '*'

            while !self.is_at_end() {
                if self.peek() == Some('*') && self.peek_next() == Some('/') {
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

    // --- Char Navigation Helpers ---

    fn is_at_end(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&self) -> Option<char> {
        self.chars.clone().next()
    }

    fn peek_next(&self) -> Option<char> {
        let mut iter = self.chars.clone();
        iter.next();
        iter.next()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.chars.next()?;
        self.pos += c.len_utf8();

        Some(c)
    }

    fn advance_while<F>(&mut self, predicate: F)
    where
        F: Fn(char) -> bool,
    {
        while self.peek().is_some_and(&predicate) {
            self.advance();
        }
    }

    #[inline]
    fn slice(&self, start: usize, end: usize) -> &'src str {
        &self.source[start..end]
    }
}
