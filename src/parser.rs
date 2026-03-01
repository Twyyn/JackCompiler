use crate::{
    ast::{
        Class, ClassVarDec, ClassVarKind, Kind, Parameter, ParseError, ReturnKind, SubroutineDec,
        SubroutineKind, TokenKind,
    },
    token::{Keyword, Symbol, Token},
};

#[derive(Debug, Clone, PartialEq)]
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
    #[rustfmt::skip]
    fn parse_kind(&mut self) -> Result<Kind<'t>, ParseError<'t>> {
        match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Int)     => Ok(Kind::Int),
                TokenKind::Keyword(Keyword::Char)    => Ok(Kind::Char),
                TokenKind::Keyword(Keyword::Boolean) => Ok(Kind::Boolean),

                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    pub fn parse_class(&mut self) -> Result<Class<'t>, ParseError<'t>> {
        self.expect(TokenKind::Keyword(Keyword::Class))?;

        let name = self.expect_identifier()?;

        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while let Some(token) = self.peek() {
            if matches!(
                token.kind,
                TokenKind::Keyword(Keyword::Static | Keyword::Field)
            ) {
                variables.push(self.parse_class_var_dec()?);
            } else {
                break;
            }
        }

        let mut subroutines = Vec::new();
        while let Some(token) = self.peek() {
            if matches!(
                token.kind,
                TokenKind::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method)
            ) {
                subroutines.push(self.parse_subroutine_dec()?);
            } else {
                break;
            }
        }

        self.expect(TokenKind::Symbol(Symbol::RightBrace))?;

        Ok(Class {
            name,
            variables,
            subroutines,
        })
    }

    fn parse_class_var_dec(&mut self) -> Result<ClassVarDec<'t>, ParseError<'t>> {
        let var_kind = match self.advance() {
            Some(token) if token.kind == TokenKind::Keyword(Keyword::Static) => {
                ClassVarKind::Static
            }
            Some(token) if token.kind == TokenKind::Keyword(Keyword::Field) => ClassVarKind::Field,
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };

        let kind = self.parse_kind()?;

        let mut names = vec![self.expect_identifier()?];
        while let Some(token) = self.peek() {
            if token.kind == TokenKind::Symbol(Symbol::Comma) {
                self.advance();
                names.push(self.expect_identifier()?);
            } else {
                break;
            }
        }

        self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

        Ok(ClassVarDec {
            var_kind,
            kind,
            names,
        })
    }

    fn parse_subroutine_dec(&mut self) -> Result<SubroutineDec<'t>, ParseError<'t>> {
        let kind = match self.advance() {
            Some(token) if token.kind == TokenKind::Keyword(Keyword::Constructor) => {
                SubroutineKind::Constructor
            }
            Some(token) if token.kind == TokenKind::Keyword(Keyword::Function) => {
                SubroutineKind::Function
            }
            Some(token) if token.kind == TokenKind::Keyword(Keyword::Method) => {
                SubroutineKind::Method
            }
            Some(token) => return Err(ParseError::UnexpectedToken(token)),
            None => return Err(ParseError::UnexpectedEof),
        };

        let return_kind = if self.peek().map(|t| t.kind) == Some(TokenKind::Keyword(Keyword::Void))
        {
            self.advance();
            ReturnKind::Void
        } else {
            ReturnKind::Kind(self.parse_kind()?)
        };

        let name = self.expect_identifier()?;

        self.expect(TokenKind::Symbol(Symbol::LeftParen))?;

        let mut parameters: Vec<Parameter> = Vec::new();
        while self.peek().map(|token| token.kind) != Some(TokenKind::Symbol(Symbol::RightParen)) {
            let kind = self.parse_kind()?;
            let name = self.expect_identifier()?;

            parameters.push(Parameter { kind, name });

            if self.peek().map(|token| token.kind) == Some(TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
            } else {
                break;
            }
        }

        self.expect(TokenKind::Symbol(Symbol::RightParen))?;

        Ok(SubroutineDec {
            kind,
            return_kind,
            name,
            parameters,
            body: todo!(),
        })
    }

    // --- Token Navigation Helpers ---
    fn has_more_tokens(&self) -> bool {
        self.pos < self.tokens.len()
    }

    fn peek(&self) -> Option<Token<'t>> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<Token<'t>> {
        if self.has_more_tokens() {
            let token = self.tokens[self.pos];
            self.pos += 1;
            Some(token)
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind<'t>) -> Result<Token<'t>, ParseError<'t>> {
        match self.advance() {
            Some(token) if token.kind == kind => Ok(token),
            Some(token) => Err(ParseError::UnexpectedToken(token)),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn expect_identifier(&mut self) -> Result<&'t str, ParseError<'t>> {
        match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Identifier(name) => Ok(name),
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }
}
