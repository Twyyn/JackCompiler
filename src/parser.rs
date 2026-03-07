use crate::parse::{
    Class, ClassVarDec, ClassVarKind, DoStatement, Expression, IfStatement, KeywordConstant, Kind,
    LetStatement, Operation, Parameter, ParseError, ReturnKind, ReturnStatement, Statement,
    SubroutineBody, SubroutineCall, SubroutineDec, SubroutineKind, Term, UnaryOperation, VarDec,
    WhileStatement,
};

use crate::token::{Keyword, Symbol, Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser<'p> {
    tokens: Vec<Token<'p>>,
    position: usize,
}

impl<'t> Parser<'t> {
    #[must_use]
    pub fn new(tokens: Vec<Token<'t>>) -> Self {
        Self {
            tokens,
            position: 0,
        }
    }

    /// Parses the entire token stream into a list of class declarations.
    ///
    /// Consumes tokens until the stream is exhausted, treating each
    /// top-level construct as a Jack class.
    ///
    /// # Errors
    ///
    /// Propagates any [`ParseError`] raised by [`parse_class`](Self::parse_class),
    /// which includes unexpected or missing tokens at any level of the grammar.
    pub fn parse(&mut self) -> Result<Vec<Class<'t>>, ParseError<'t>> {
        let mut classes = Vec::new();
        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }
        Ok(classes)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
            || self.peek_matches(|kind| matches!(kind, TokenKind::Eof))
    }

    // ── Token Navigation ─────────────────────────────────────────────

    fn peek(&self) -> Option<Token<'t>> {
        self.tokens.get(self.position).copied()
    }

    fn peek_is(&self, expected: TokenKind) -> bool {
        self.peek_matches(|kind| *kind == expected)
    }

    fn peek_matches(&self, f: impl FnOnce(&TokenKind) -> bool) -> bool {
        self.peek().is_some_and(|token| f(&token.kind))
    }

    fn advance(&mut self) -> Option<Token<'t>> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position];
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn advance_or_end(&mut self) -> Result<Token<'t>, ParseError<'t>> {
        self.advance().ok_or(ParseError::UnexpectedEof)
    }

    fn expect(&mut self, kind: TokenKind<'t>) -> Result<Token<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        match token {
            token if token.kind == kind => Ok(token),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn expect_identifier(&mut self) -> Result<&'t str, ParseError<'t>> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Type Parsing ─────────────────────────────────────────────────

    #[rustfmt::skip]
    fn parse_kind(&mut self) -> Result<Kind<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Keyword(Keyword::Int)     => Ok(Kind::Int),
            TokenKind::Keyword(Keyword::Char)    => Ok(Kind::Char),
            TokenKind::Keyword(Keyword::Boolean) => Ok(Kind::Boolean),
            TokenKind::Identifier(name)          => Ok(Kind::Class(name)),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Comma-Separated Lists ────────────────────────────────────────

    fn parse_parameter(&mut self) -> Result<Parameter<'t>, ParseError<'t>> {
        let kind = self.parse_kind()?;
        let name = self.expect_identifier()?;
        Ok(Parameter { kind, name })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter<'t>>, ParseError<'t>> {
        self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
        let mut params = Vec::new();
        if !self.peek_is(TokenKind::Symbol(Symbol::RightParen)) {
            params.push(self.parse_parameter()?);
            while self.peek_is(TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
                params.push(self.parse_parameter()?);
            }
        }
        self.expect(TokenKind::Symbol(Symbol::RightParen))?;
        Ok(params)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression<'t>>, ParseError<'t>> {
        let mut args = Vec::new();
        if !self.peek_is(TokenKind::Symbol(Symbol::RightParen)) {
            args.push(self.parse_expression()?);
            while self.peek_is(TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
                args.push(self.parse_expression()?);
            }
        }
        Ok(args)
    }

    // ── Expression & Term Parsing ────────────────────────────────────

    fn parse_expression(&mut self) -> Result<Expression<'t>, ParseError<'t>> {
        let term = self.parse_term()?;
        let mut operations = Vec::new();

        while let Some(operation) = self.peek().and_then(|token| match token.kind {
            TokenKind::Symbol(symbol) => symbol.as_binary_operation(),
            _ => None,
        }) {
            self.advance();
            operations.push((operation, self.parse_term()?));
        }

        Ok(Expression { term, operations })
    }

    fn parse_term(&mut self) -> Result<Term<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::IntegerConstant(integer) => Ok(Term::IntegerConstant(integer)),
            TokenKind::StringConstant(string) => Ok(Term::StringConstant(string)),

            TokenKind::Keyword(keyword) => match keyword {
                Keyword::True => Ok(Term::KeywordConstant(KeywordConstant::True)),
                Keyword::False => Ok(Term::KeywordConstant(KeywordConstant::False)),
                Keyword::Null => Ok(Term::KeywordConstant(KeywordConstant::Null)),
                Keyword::This => Ok(Term::KeywordConstant(KeywordConstant::This)),
                _ => Err(ParseError::UnexpectedToken(token)),
            },

            TokenKind::Identifier(name) => {
                if self.peek_is(TokenKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(TokenKind::Symbol(Symbol::RightBracket))?;
                    Ok(Term::ArrayAccess(name, Box::new(index)))
                } else if self.peek_is(TokenKind::Symbol(Symbol::LeftParen))
                    || self.peek_is(TokenKind::Symbol(Symbol::Dot))
                {
                    Ok(Term::SubroutineCall(self.parse_subroutine_call(name)?))
                } else {
                    Ok(Term::Variable(name))
                }
            }

            TokenKind::Symbol(Symbol::LeftParen) => {
                let expr = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                Ok(Term::Grouped(Box::new(expr)))
            }
            TokenKind::Symbol(Symbol::Minus) => Ok(Term::Unary(
                UnaryOperation::Minus,
                Box::new(self.parse_term()?),
            )),
            TokenKind::Symbol(Symbol::Tilde) => Ok(Term::Unary(
                UnaryOperation::Tilde,
                Box::new(self.parse_term()?),
            )),

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_subroutine_call(
        &mut self,
        first: &'t str,
    ) -> Result<SubroutineCall<'t>, ParseError<'t>> {
        let (receiver, name) = if self.peek_is(TokenKind::Symbol(Symbol::Dot)) {
            self.advance();
            (Some(first), self.expect_identifier()?)
        } else {
            (None, first)
        };

        self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
        let arguments = self.parse_expression_list()?;
        self.expect(TokenKind::Symbol(Symbol::RightParen))?;

        Ok(SubroutineCall {
            name,
            receiver,
            arguments,
        })
    }

    // ── Statement & Block Parsing ────────────────────────────────────

    fn parse_block(&mut self) -> Result<Vec<Statement<'t>>, ParseError<'t>> {
        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.peek_is(TokenKind::Symbol(Symbol::RightBrace)) {
            statements.push(self.parse_statement()?);
        }
        self.expect(TokenKind::Symbol(Symbol::RightBrace))?;
        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Keyword(Keyword::Let) => {
                let name = self.expect_identifier()?;
                let index = if self.peek_is(TokenKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = Some(self.parse_expression()?);
                    self.expect(TokenKind::Symbol(Symbol::RightBracket))?;
                    index
                } else {
                    None
                };

                self.expect(TokenKind::Symbol(Symbol::Equal))?;
                let expression = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Let(LetStatement::new(name, index, expression)))
            }

            TokenKind::Keyword(Keyword::If) => {
                self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::RightParen))?;

                let if_body = self.parse_block()?;
                let else_body = if self.peek_is(TokenKind::Keyword(Keyword::Else)) {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };

                Ok(Statement::If(IfStatement::new(
                    condition, if_body, else_body,
                )))
            }

            TokenKind::Keyword(Keyword::While) => {
                self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                let body = self.parse_block()?;

                Ok(Statement::While(WhileStatement::new(condition, body)))
            }

            TokenKind::Keyword(Keyword::Do) => {
                let name = self.expect_identifier()?;
                let call = self.parse_subroutine_call(name)?;
                self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Do(DoStatement::new(call)))
            }

            TokenKind::Keyword(Keyword::Return) => {
                let expression = if self.peek_is(TokenKind::Symbol(Symbol::Semicolon)) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Return(ReturnStatement::new(expression)))
            }

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Declaration Parsing ──────────────────────────────────────────

    fn parse_var_dec(&mut self) -> Result<VarDec<'t>, ParseError<'t>> {
        self.expect(TokenKind::Keyword(Keyword::Var))?;
        self.advance(); // consume 'var'
        let kind = self.parse_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(TokenKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(TokenKind::Symbol(Symbol::Semicolon))?;
        Ok(VarDec { kind, names })
    }

    fn parse_class_var_dec(&mut self) -> Result<ClassVarDec<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        let var_kind = match token.kind {
            TokenKind::Keyword(Keyword::Static) => ClassVarKind::Static,
            TokenKind::Keyword(Keyword::Field) => ClassVarKind::Field,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let kind = self.parse_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(TokenKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

        Ok(ClassVarDec {
            var_kind,
            kind,
            names,
        })
    }

    fn parse_subroutine_dec(&mut self) -> Result<SubroutineDec<'t>, ParseError<'t>> {
        let token = self.advance_or_end()?;
        let kind = match token.kind {
            TokenKind::Keyword(Keyword::Constructor) => SubroutineKind::Constructor,
            TokenKind::Keyword(Keyword::Function) => SubroutineKind::Function,
            TokenKind::Keyword(Keyword::Method) => SubroutineKind::Method,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let return_kind = if self.peek_is(TokenKind::Keyword(Keyword::Void)) {
            self.advance();
            ReturnKind::Void
        } else {
            ReturnKind::Kind(self.parse_kind()?)
        };

        let name = self.expect_identifier()?;
        let parameters = self.parse_parameter_list()?;

        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while self.peek_is(TokenKind::Keyword(Keyword::Var)) {
            variables.push(self.parse_var_dec()?);
        }

        let mut statements = Vec::new();
        while !self.is_at_end() && !self.peek_is(TokenKind::Symbol(Symbol::RightBrace)) {
            statements.push(self.parse_statement()?);
        }

        self.expect(TokenKind::Symbol(Symbol::RightBrace))?;

        Ok(SubroutineDec {
            kind,
            return_kind,
            name,
            parameters,
            body: SubroutineBody {
                variables,
                statements,
            },
        })
    }

    fn parse_class(&mut self) -> Result<Class<'t>, ParseError<'t>> {
        self.expect(TokenKind::Keyword(Keyword::Class))?;
        let name = self.expect_identifier()?;
        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while self.peek_matches(|kind| {
            matches!(kind, TokenKind::Keyword(Keyword::Static | Keyword::Field))
        }) {
            variables.push(self.parse_class_var_dec()?);
        }

        let mut subroutines = Vec::new();
        while self.peek_matches(|kind| {
            matches!(
                kind,
                TokenKind::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method)
            )
        }) {
            subroutines.push(self.parse_subroutine_dec()?);
        }

        self.expect(TokenKind::Symbol(Symbol::RightBrace))?;

        Ok(Class {
            name,
            variables,
            subroutines,
        })
    }
}

impl Symbol {
    #[must_use]
    pub fn as_binary_operation(self) -> Option<Operation> {
        match self {
            Symbol::Plus => Some(Operation::Add),
            Symbol::Minus => Some(Operation::Sub),
            Symbol::Star => Some(Operation::Mul),
            Symbol::Slash => Some(Operation::Div),
            Symbol::Ampersand => Some(Operation::And),
            Symbol::Pipe => Some(Operation::Or),
            Symbol::GreaterThan => Some(Operation::GreaterThan),
            Symbol::LessThan => Some(Operation::LessThan),
            Symbol::Equal => Some(Operation::Equal),
            _ => None,
        }
    }
}
