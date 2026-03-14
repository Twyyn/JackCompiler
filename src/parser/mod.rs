pub mod ast;
pub mod error;

use crate::lexer::token::kind::Identifier;
use crate::lexer::token::{Keyword, Symbol, Token, TokenKind};
use crate::parser::ast::declaration::{
    Class, ClassVarDec, ClassVarKind, DataKind, Parameter, ReturnKind, SubroutineBody,
    SubroutineCall, SubroutineDec, SubroutineKind, VarDec,
};
use crate::parser::ast::expression::{Expr, KeywordConstant, Term, UnaryOp};
use crate::parser::ast::statement::{DoStmt, IfStmt, LetStmt, ReturnStmt, Statement, WhileStmt};
use crate::parser::error::ParseError;

// ── Parse Result ────────────────────────────────────────
type ParseResult<T> = std::result::Result<T, ParseError>;

#[derive(Debug)]
pub struct Parser<'t> {
    tokens: &'t [Token],
    position: usize,
}

impl<'t> Parser<'t> {
    #[must_use]
    pub fn new(tokens: &'t [Token]) -> Self {
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
    pub fn parse(&mut self) -> ParseResult<Vec<Class>> {
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

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn peek_is(&self, expected: &TokenKind) -> bool {
        self.peek_matches(|kind| *kind == *expected)
    }

    fn peek_matches(&self, f: impl FnOnce(&TokenKind) -> bool) -> bool {
        self.peek().is_some_and(|token| f(&token.kind))
    }

    fn advance(&mut self) -> Option<Token> {
        if self.position < self.tokens.len() {
            let token = self.tokens[self.position].clone();
            self.position += 1;
            Some(token)
        } else {
            None
        }
    }

    fn advance_or_end(&mut self) -> ParseResult<Token> {
        self.advance().ok_or(ParseError::UnexpectedEof)
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<Token, ParseError> {
        let token = self.advance_or_end()?;
        match token {
            token if token.kind == *kind => Ok(token),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn expect_identifier(&mut self) -> Result<Identifier, ParseError> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── DataKind Parsing ─────────────────────────────────────────────────

    fn parse_kind(&mut self) -> ParseResult<DataKind> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Keyword(Keyword::Int) => Ok(DataKind::Int),
            TokenKind::Keyword(Keyword::Char) => Ok(DataKind::Char),
            TokenKind::Keyword(Keyword::Boolean) => Ok(DataKind::Boolean),
            TokenKind::Identifier(name) => Ok(DataKind::Class(name)),

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Comma-Separated Lists ────────────────────────────────────────

    fn parse_parameter(&mut self) -> ParseResult<Parameter> {
        let kind = self.parse_kind()?;
        let name = self.expect_identifier()?;

        Ok(Parameter { name, kind })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ParseError> {
        self.expect(&TokenKind::Symbol(Symbol::LeftParen))?;
        let mut params = Vec::new();
        if !self.peek_is(&TokenKind::Symbol(Symbol::RightParen)) {
            params.push(self.parse_parameter()?);
            while self.peek_is(&TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
                params.push(self.parse_parameter()?);
            }
        }
        self.expect(&TokenKind::Symbol(Symbol::RightParen))?;

        Ok(params)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expr>, ParseError> {
        let mut args = Vec::new();
        if !self.peek_is(&TokenKind::Symbol(Symbol::RightParen)) {
            args.push(self.parse_expression()?);
            while self.peek_is(&TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
                args.push(self.parse_expression()?);
            }
        }

        Ok(args)
    }

    // ── Expression & Term Parsing ────────────────────────────────────

    fn parse_expression(&mut self) -> ParseResult<Expr> {
        let term = self.parse_term()?;
        let mut operations = Vec::new();

        while let Some(op) = self.peek().and_then(|token| match token.kind {
            TokenKind::Symbol(symbol) => symbol.as_binary_operation(),
            _ => None,
        }) {
            self.advance();
            operations.push((op, self.parse_term()?));
        }

        Ok(Expr {
            term,
            op: operations,
        })
    }

    #[allow(clippy::cast_possible_truncation)]
    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::IntegerConstant(int) => Ok(Term::IntegerConstant(int as u16)),
            TokenKind::StringConstant(string) => Ok(Term::StringConstant(string)),

            TokenKind::Keyword(keyword) => match keyword {
                Keyword::True => Ok(Term::KeywordConstant(KeywordConstant::True)),
                Keyword::False => Ok(Term::KeywordConstant(KeywordConstant::False)),
                Keyword::Null => Ok(Term::KeywordConstant(KeywordConstant::Null)),
                Keyword::This => Ok(Term::KeywordConstant(KeywordConstant::This)),
                _ => Err(ParseError::UnexpectedToken(token)),
            },

            TokenKind::Identifier(name) => {
                if self.peek_is(&TokenKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(&TokenKind::Symbol(Symbol::RightBracket))?;
                    Ok(Term::ArrayAccess(name, Box::new(index)))
                } else if self.peek_is(&TokenKind::Symbol(Symbol::LeftParen))
                    || self.peek_is(&TokenKind::Symbol(Symbol::Dot))
                {
                    Ok(Term::SubroutineCall(self.parse_subroutine_call(&name)?))
                } else {
                    Ok(Term::Variable(name))
                }
            }

            TokenKind::Symbol(Symbol::LeftParen) => {
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::Symbol(Symbol::RightParen))?;
                Ok(Term::Grouped(Box::new(expr)))
            }
            TokenKind::Symbol(Symbol::Minus) => {
                Ok(Term::Unary(UnaryOp::Minus, Box::new(self.parse_term()?)))
            }
            TokenKind::Symbol(Symbol::Tilde) => {
                Ok(Term::Unary(UnaryOp::Tilde, Box::new(self.parse_term()?)))
            }

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_subroutine_call(&mut self, first: &str) -> Result<SubroutineCall, ParseError> {
        let (receiver, name) = if self.peek_is(&TokenKind::Symbol(Symbol::Dot)) {
            self.advance();
            (Some(first.into()), self.expect_identifier()?)
        } else {
            (None, first.into())
        };

        self.expect(&TokenKind::Symbol(Symbol::LeftParen))?;
        let args = self.parse_expression_list()?;
        self.expect(&TokenKind::Symbol(Symbol::RightParen))?;

        Ok(SubroutineCall {
            name,
            receiver,
            args,
        })
    }

    // ── Statement & Block Parsing ────────────────────────────────────

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect(&TokenKind::Symbol(Symbol::LeftBrace))?;
        let statements = self.parse_statement_list()?;
        self.expect(&TokenKind::Symbol(Symbol::RightBrace))?;
        Ok(statements)
    }
    fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.peek_is(&TokenKind::Symbol(Symbol::RightBrace)) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.advance_or_end()?;
        match token.kind {
            TokenKind::Keyword(Keyword::Let) => {
                let name = self.expect_identifier()?;
                let index = if self.peek_is(&TokenKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = Some(self.parse_expression()?);
                    self.expect(&TokenKind::Symbol(Symbol::RightBracket))?;
                    index
                } else {
                    None
                };

                self.expect(&TokenKind::Symbol(Symbol::Equal))?;
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Let(LetStmt { name, index, expr }))
            }

            TokenKind::Keyword(Keyword::If) => {
                self.expect(&TokenKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(&TokenKind::Symbol(Symbol::RightParen))?;

                let if_body = self.parse_block()?;
                let else_body = if self.peek_is(&TokenKind::Keyword(Keyword::Else)) {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };

                Ok(Statement::If(IfStmt {
                    condition,
                    if_body,
                    else_body,
                }))
            }

            TokenKind::Keyword(Keyword::While) => {
                self.expect(&TokenKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(&TokenKind::Symbol(Symbol::RightParen))?;
                let body = self.parse_block()?;

                Ok(Statement::While(WhileStmt { condition, body }))
            }

            TokenKind::Keyword(Keyword::Do) => {
                let name = self.expect_identifier()?;
                let subroutine_call = self.parse_subroutine_call(&name)?;
                self.expect(&TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Do(DoStmt { subroutine_call }))
            }

            TokenKind::Keyword(Keyword::Return) => {
                let expr = if self.peek_is(&TokenKind::Symbol(Symbol::Semicolon)) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(&TokenKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Return(ReturnStmt { expr }))
            }

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Declaration Parsing ──────────────────────────────────────────

    fn parse_var_dec(&mut self) -> ParseResult<VarDec> {
        self.expect(&TokenKind::Keyword(Keyword::Var))?;
        let kind = self.parse_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(&TokenKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(&TokenKind::Symbol(Symbol::Semicolon))?;

        Ok(VarDec { names, kind })
    }

    fn parse_class_var_dec(&mut self) -> Result<ClassVarDec, ParseError> {
        let token = self.advance_or_end()?;
        let var_kind = match token.kind {
            TokenKind::Keyword(Keyword::Static) => ClassVarKind::Static,
            TokenKind::Keyword(Keyword::Field) => ClassVarKind::Field,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let kind = self.parse_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(&TokenKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(&TokenKind::Symbol(Symbol::Semicolon))?;

        Ok(ClassVarDec {
            names,
            kind,
            var_kind,
        })
    }

    fn parse_subroutine_dec(&mut self) -> Result<SubroutineDec, ParseError> {
        let token = self.advance_or_end()?;
        let kind = match token.kind {
            TokenKind::Keyword(Keyword::Constructor) => SubroutineKind::Constructor,
            TokenKind::Keyword(Keyword::Function) => SubroutineKind::Function,
            TokenKind::Keyword(Keyword::Method) => SubroutineKind::Method,

            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let return_kind = if self.peek_is(&TokenKind::Keyword(Keyword::Void)) {
            self.advance();
            ReturnKind::Void
        } else {
            ReturnKind::Kind(self.parse_kind()?)
        };

        let name = self.expect_identifier()?;
        let parameters = self.parse_parameter_list()?;

        self.expect(&TokenKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while self.peek_is(&TokenKind::Keyword(Keyword::Var)) {
            variables.push(self.parse_var_dec()?);
        }

        let statements = self.parse_statement_list()?;

        self.expect(&TokenKind::Symbol(Symbol::RightBrace))?;

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

    fn parse_class(&mut self) -> ParseResult<Class> {
        self.expect(&TokenKind::Keyword(Keyword::Class))?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::Symbol(Symbol::LeftBrace))?;

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

        self.expect(&TokenKind::Symbol(Symbol::RightBrace))?;

        Ok(Class {
            name,
            variables,
            subroutines,
        })
    }
}