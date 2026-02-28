use crate::{
    ast::{ParseError, TokenKind},
    token::Token,
};

pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
    pos: usize,
}

impl<'t> Parser<'t> {
    #[must_use]
    pub fn new(tokens: Vec<Token<'t>>) -> Self {
        Self { tokens, pos: 0 }
    }

    // --- Parse Helpers ---

    // --- Token Navigation Helpers ---
    fn has_more_tokens(&self) -> bool {
        self.pos < self.tokens.len()
    }

    fn peek(&self) -> Option<Token<'_>> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<Token<'_>> {
        if self.has_more_tokens() {
            let token = self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind) -> Result<Token<'_>, ParseError<'_>> {
        match self.advance() {
            Some(token) if token.kind == kind => Ok(token),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEof),
        }
    }
}
