pub mod ast;
pub mod error;

use crate::lexer::token::r#type::Identifier;
use crate::lexer::token::{Keyword, Symbol, Token, TokenTypeKind};
use crate::parser::ast::expressions::{Expression, KeywordConstant, Term, UnaryOperation};
use crate::parser::ast::nodes::{
    Class, ClassVarDec, ClassVarTypeKind, Parameter, ReturnTypeKind, SubroutineBody,
    SubroutineCall, SubroutineDec, SubroutineTypeKind, TypeKind, VarDec,
};
use crate::parser::ast::statements::{
    DoStatement, IfStatement, LetStatement, ReturnStatement, Statement, WhileStatement,
};
use crate::parser::error::ParseError;

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    position: usize,
}

impl Parser {
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
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
    pub fn parse(&mut self) -> Result<Vec<Class>, ParseError> {
        let mut classes = Vec::new();
        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }

        Ok(classes)
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
            || self.peek_matches(|kind| matches!(kind, TokenTypeKind::Eof))
    }

    // ── Token Navigation ─────────────────────────────────────────────

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn peek_is(&self, expected: &TokenTypeKind) -> bool {
        self.peek_matches(|kind| *kind == *expected)
    }

    fn peek_matches(&self, f: impl FnOnce(&TokenTypeKind) -> bool) -> bool {
        self.peek().is_some_and(|token| f(&token.token_type_kind))
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

    fn advance_or_end(&mut self) -> Result<Token, ParseError> {
        self.advance().ok_or(ParseError::UnexpectedEof)
    }

    fn expect(&mut self, token_type_kind: &TokenTypeKind) -> Result<Token, ParseError> {
        let token = self.advance_or_end()?;
        match token {
            token if token.token_type_kind == *token_type_kind => Ok(token),
            token => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn expect_identifier(&mut self) -> Result<Identifier, ParseError> {
        let token = self.advance_or_end()?;
        match token.token_type_kind {
            TokenTypeKind::Identifier(name) => Ok(name),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Type Parsing ─────────────────────────────────────────────────

    #[rustfmt::skip]
    fn parse_type_kind(&mut self) -> Result<TypeKind, ParseError> {
        let token = self.advance_or_end()?;
        match token.token_type_kind {
           TokenTypeKind::Keyword(Keyword::Int)     => Ok(TypeKind::Int),
           TokenTypeKind::Keyword(Keyword::Char)    => Ok(TypeKind::Char),
           TokenTypeKind::Keyword(Keyword::Boolean) => Ok(TypeKind::Boolean),
           TokenTypeKind::Identifier(name)          => Ok(TypeKind::Class(name)),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Comma-Separated Lists ────────────────────────────────────────

    fn parse_parameter(&mut self) -> Result<Parameter, ParseError> {
        let type_kind = self.parse_type_kind()?;
        let name = self.expect_identifier()?;

        Ok(Parameter { type_kind, name })
    }

    fn parse_parameter_list(&mut self) -> Result<Vec<Parameter>, ParseError> {
        self.expect(&TokenTypeKind::Symbol(Symbol::LeftParen))?;
        let mut params = Vec::new();
        if !self.peek_is(&TokenTypeKind::Symbol(Symbol::RightParen)) {
            params.push(self.parse_parameter()?);
            while self.peek_is(&TokenTypeKind::Symbol(Symbol::Comma)) {
                self.advance();
                params.push(self.parse_parameter()?);
            }
        }
        self.expect(&TokenTypeKind::Symbol(Symbol::RightParen))?;

        Ok(params)
    }

    fn parse_expression_list(&mut self) -> Result<Vec<Expression>, ParseError> {
        let mut args = Vec::new();
        if !self.peek_is(&TokenTypeKind::Symbol(Symbol::RightParen)) {
            args.push(self.parse_expression()?);
            while self.peek_is(&TokenTypeKind::Symbol(Symbol::Comma)) {
                self.advance();
                args.push(self.parse_expression()?);
            }
        }

        Ok(args)
    }

    // ── Expression & Term Parsing ────────────────────────────────────

    fn parse_expression(&mut self) -> Result<Expression, ParseError> {
        let term = self.parse_term()?;
        let mut operations = Vec::new();

        while let Some(operation) = self.peek().and_then(|token| match token.token_type_kind {
            TokenTypeKind::Symbol(symbol) => symbol.as_binary_operation(),
            _ => None,
        }) {
            self.advance();
            operations.push((operation, self.parse_term()?));
        }

        Ok(Expression { term, operations })
    }

    fn parse_term(&mut self) -> Result<Term, ParseError> {
        let token = self.advance_or_end()?;
        match token.token_type_kind {
            TokenTypeKind::IntegerConstant(integer) => Ok(Term::IntegerConstant(integer)),
            TokenTypeKind::StringConstant(string) => Ok(Term::StringConstant(string)),

            TokenTypeKind::Keyword(keyword) => match keyword {
                Keyword::True => Ok(Term::KeywordConstant(KeywordConstant::True)),
                Keyword::False => Ok(Term::KeywordConstant(KeywordConstant::False)),
                Keyword::Null => Ok(Term::KeywordConstant(KeywordConstant::Null)),
                Keyword::This => Ok(Term::KeywordConstant(KeywordConstant::This)),
                _ => Err(ParseError::UnexpectedToken(token)),
            },

            TokenTypeKind::Identifier(name) => {
                if self.peek_is(&TokenTypeKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = self.parse_expression()?;
                    self.expect(&TokenTypeKind::Symbol(Symbol::RightBracket))?;
                    Ok(Term::ArrayAccess(name, Box::new(index)))
                } else if self.peek_is(&TokenTypeKind::Symbol(Symbol::LeftParen))
                    || self.peek_is(&TokenTypeKind::Symbol(Symbol::Dot))
                {
                    Ok(Term::SubroutineCall(self.parse_subroutine_call(&name)?))
                } else {
                    Ok(Term::Variable(name))
                }
            }

            TokenTypeKind::Symbol(Symbol::LeftParen) => {
                let expr = self.parse_expression()?;
                self.expect(&TokenTypeKind::Symbol(Symbol::RightParen))?;
                Ok(Term::Grouped(Box::new(expr)))
            }
            TokenTypeKind::Symbol(Symbol::Minus) => Ok(Term::Unary(
                UnaryOperation::Minus,
                Box::new(self.parse_term()?),
            )),
            TokenTypeKind::Symbol(Symbol::Tilde) => Ok(Term::Unary(
                UnaryOperation::Tilde,
                Box::new(self.parse_term()?),
            )),

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_subroutine_call(&mut self, first: &str) -> Result<SubroutineCall, ParseError> {
        let (receiver, name) = if self.peek_is(&TokenTypeKind::Symbol(Symbol::Dot)) {
            self.advance();
            (Some(first.into()), self.expect_identifier()?)
        } else {
            (None, first.into())
        };

        self.expect(&TokenTypeKind::Symbol(Symbol::LeftParen))?;
        let arguments = self.parse_expression_list()?;
        self.expect(&TokenTypeKind::Symbol(Symbol::RightParen))?;

        Ok(SubroutineCall {
            name,
            receiver,
            arguments,
        })
    }

    // ── Statement & Block Parsing ────────────────────────────────────

    fn parse_block(&mut self) -> Result<Vec<Statement>, ParseError> {
        self.expect(&TokenTypeKind::Symbol(Symbol::LeftBrace))?;
        let statements = self.parse_statement_list()?;
        self.expect(&TokenTypeKind::Symbol(Symbol::RightBrace))?;
        Ok(statements)
    }
    fn parse_statement_list(&mut self) -> Result<Vec<Statement>, ParseError> {
        let mut statements = Vec::new();
        while !self.is_at_end() && !self.peek_is(&TokenTypeKind::Symbol(Symbol::RightBrace)) {
            statements.push(self.parse_statement()?);
        }

        Ok(statements)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        let token = self.advance_or_end()?;
        match token.token_type_kind {
            TokenTypeKind::Keyword(Keyword::Let) => {
                let name = self.expect_identifier()?;
                let index = if self.peek_is(&TokenTypeKind::Symbol(Symbol::LeftBracket)) {
                    self.advance();
                    let index = Some(self.parse_expression()?);
                    self.expect(&TokenTypeKind::Symbol(Symbol::RightBracket))?;
                    index
                } else {
                    None
                };

                self.expect(&TokenTypeKind::Symbol(Symbol::Equal))?;
                let expression = self.parse_expression()?;
                self.expect(&TokenTypeKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Let(LetStatement {
                    name,
                    index,
                    expression,
                }))
            }

            TokenTypeKind::Keyword(Keyword::If) => {
                self.expect(&TokenTypeKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(&TokenTypeKind::Symbol(Symbol::RightParen))?;

                let if_body = self.parse_block()?;
                let else_body = if self.peek_is(&TokenTypeKind::Keyword(Keyword::Else)) {
                    self.advance();
                    Some(self.parse_block()?)
                } else {
                    None
                };

                Ok(Statement::If(IfStatement {
                    condition,
                    if_body,
                    else_body,
                }))
            }

            TokenTypeKind::Keyword(Keyword::While) => {
                self.expect(&TokenTypeKind::Symbol(Symbol::LeftParen))?;
                let condition = self.parse_expression()?;
                self.expect(&TokenTypeKind::Symbol(Symbol::RightParen))?;
                let body = self.parse_block()?;

                Ok(Statement::While(WhileStatement { condition, body }))
            }

            TokenTypeKind::Keyword(Keyword::Do) => {
                let name = self.expect_identifier()?;
                let subroutine_call = self.parse_subroutine_call(&name)?;
                self.expect(&TokenTypeKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Do(DoStatement { subroutine_call }))
            }

            TokenTypeKind::Keyword(Keyword::Return) => {
                let expression = if self.peek_is(&TokenTypeKind::Symbol(Symbol::Semicolon)) {
                    None
                } else {
                    Some(self.parse_expression()?)
                };
                self.expect(&TokenTypeKind::Symbol(Symbol::Semicolon))?;

                Ok(Statement::Return(ReturnStatement { expression }))
            }

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Declaration Parsing ──────────────────────────────────────────

    fn parse_var_dec(&mut self) -> Result<VarDec, ParseError> {
        self.expect(&TokenTypeKind::Keyword(Keyword::Var))?;
        let type_kind = self.parse_type_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(&TokenTypeKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(&TokenTypeKind::Symbol(Symbol::Semicolon))?;

        Ok(VarDec { type_kind, names })
    }

    fn parse_class_var_dec(&mut self) -> Result<ClassVarDec, ParseError> {
        let token = self.advance_or_end()?;
        let var_kind = match token.token_type_kind {
            TokenTypeKind::Keyword(Keyword::Static) => ClassVarTypeKind::Static,
            TokenTypeKind::Keyword(Keyword::Field) => ClassVarTypeKind::Field,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let type_kind = self.parse_type_kind()?;
        let mut names = vec![self.expect_identifier()?];
        while self.peek_is(&TokenTypeKind::Symbol(Symbol::Comma)) {
            self.advance();
            names.push(self.expect_identifier()?);
        }
        self.expect(&TokenTypeKind::Symbol(Symbol::Semicolon))?;

        Ok(ClassVarDec {
            var_kind,
            type_kind,
            names,
        })
    }

    fn parse_subroutine_dec(&mut self) -> Result<SubroutineDec, ParseError> {
        let token = self.advance_or_end()?;
        let type_kind = match token.token_type_kind {
            TokenTypeKind::Keyword(Keyword::Constructor) => SubroutineTypeKind::Constructor,
            TokenTypeKind::Keyword(Keyword::Function) => SubroutineTypeKind::Function,
            TokenTypeKind::Keyword(Keyword::Method) => SubroutineTypeKind::Method,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let return_type_kind = if self.peek_is(&TokenTypeKind::Keyword(Keyword::Void)) {
            self.advance();
            ReturnTypeKind::Void
        } else {
            ReturnTypeKind::TypeKind(self.parse_type_kind()?)
        };

        let name = self.expect_identifier()?;
        let parameters = self.parse_parameter_list()?;

        self.expect(&TokenTypeKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while self.peek_is(&TokenTypeKind::Keyword(Keyword::Var)) {
            variables.push(self.parse_var_dec()?);
        }

        let statements = self.parse_statement_list()?;

        self.expect(&TokenTypeKind::Symbol(Symbol::RightBrace))?;

        Ok(SubroutineDec {
            type_kind,
            return_type_kind,
            name,
            parameters,
            body: SubroutineBody {
                variables,
                statements,
            },
        })
    }

    fn parse_class(&mut self) -> Result<Class, ParseError> {
        self.expect(&TokenTypeKind::Keyword(Keyword::Class))?;
        let name = self.expect_identifier()?;
        self.expect(&TokenTypeKind::Symbol(Symbol::LeftBrace))?;

        let mut variables = Vec::new();
        while self.peek_matches(|kind| {
            matches!(
                kind,
                TokenTypeKind::Keyword(Keyword::Static | Keyword::Field)
            )
        }) {
            variables.push(self.parse_class_var_dec()?);
        }

        let mut subroutines = Vec::new();
        while self.peek_matches(|kind| {
            matches!(
                kind,
                TokenTypeKind::Keyword(Keyword::Constructor | Keyword::Function | Keyword::Method)
            )
        }) {
            subroutines.push(self.parse_subroutine_dec()?);
        }

        self.expect(&TokenTypeKind::Symbol(Symbol::RightBrace))?;

        Ok(Class {
            name,
            variables,
            subroutines,
        })
    }
}
