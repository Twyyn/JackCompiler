pub mod ast;
pub mod error;

use crate::lexer::Lexer;
use crate::lexer::token::{Token, TokenKind};
use crate::parser::ast::declaration::{
    Class, ClassVarDec, Kind, Parameter, ReturnType, SubroutineBody, SubroutineCall, SubroutineDec,
    SubroutineKind, Type, VarDec,
};
use crate::parser::ast::expression::{BinaryOp, Expr, KeywordConstant, Term, UnaryOp};
use crate::parser::ast::statement::{DoStmt, IfStmt, LetStmt, ReturnStmt, Statement, WhileStmt};
use crate::parser::error::ParseError;

#[derive(Debug)]
pub struct Parser<'src> {
    lexer: std::iter::Peekable<Lexer<'src>>,
}

impl<'src> Parser<'src> {
    #[must_use]
    pub fn new(lexer: Lexer<'src>) -> Self {
        Self {
            lexer: lexer.peekable(),
        }
    }

    /// Parses the entire token stream into a list of class declarations.
    ///
    /// # Errors
    ///
    /// Propagates any [`ParseError`] raised during parsing.
    pub fn parse(&mut self) -> Result<Vec<Class<'src>>, ParseError<'src>> {
        let mut classes = Vec::new();
        while !self.is_at_end() {
            classes.push(self.parse_class()?);
        }
        Ok(classes)
    }

    fn is_at_end(&mut self) -> bool {
        self.peek_kind() == Some(&TokenKind::Eof)
    }

    // ── Token Navigation ─────────────────────────────────────────────

    fn peek_kind(&mut self) -> Option<&TokenKind<'src>> {
        self.peek().map(|t| &t.kind)
    }

    fn peek(&mut self) -> Option<&Token<'src>> {
        self.lexer.peek().and_then(|r| r.as_ref().ok())
    }

    fn peek_is(&mut self, expected: &TokenKind) -> bool {
        self.peek_kind() == Some(expected)
    }

    fn peek_matches(&mut self, f: impl FnOnce(&TokenKind<'src>) -> bool) -> bool {
        self.peek_kind().is_some_and(f)
    }

    fn advance(&mut self) -> Result<Token<'src>, ParseError<'src>> {
        match self.lexer.next() {
            Some(Ok(token)) => Ok(token),
            Some(Err(e)) => Err(ParseError::from(e)),
            None => Err(ParseError::UnexpectedEof),
        }
    }

    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.peek_is(kind) {
            let _ = self.advance();
            true
        } else {
            false
        }
    }

    fn expect(&mut self, kind: &TokenKind) -> Result<Token<'src>, ParseError<'src>> {
        let token = self.advance()?;
        if token.kind == *kind {
            Ok(token)
        } else {
            Err(ParseError::UnexpectedToken(token))
        }
    }

    fn expect_identifier(&mut self) -> Result<&'src str, ParseError<'src>> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Identifier(name) => Ok(name),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    // ── Comma-Separated Lists ────────────────────────────────────────

    fn comma_separated<T>(
        &mut self,
        terminator: &TokenKind,
        mut parse_item: impl FnMut(&mut Self) -> Result<T, ParseError<'src>>,
    ) -> Result<Vec<T>, ParseError<'src>> {
        let mut items = Vec::new();
        if !self.peek_is(terminator) {
            items.push(parse_item(self)?);
            while self.eat(&TokenKind::Comma) {
                items.push(parse_item(self)?);
            }
        }
        Ok(items)
    }

    // ── Type Parsing ─────────────────────────────────────────────────

    #[rustfmt::skip]
    fn parse_type(&mut self) -> Result<Type<'src>, ParseError<'src>> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Int             => Ok(Type::Int),
            TokenKind::Char            => Ok(Type::Char),
            TokenKind::Boolean         => Ok(Type::Boolean),
            TokenKind::Identifier(name) => Ok(Type::Class(name)),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_return_type(&mut self) -> Result<ReturnType<'src>, ParseError<'src>> {
        if self.eat(&TokenKind::Void) {
            Ok(ReturnType::Void)
        } else {
            self.parse_type().map(ReturnType::Type)
        }
    }

    // ── Expression & Term Parsing ────────────────────────────────────

    fn parse_expression(&mut self) -> Result<Expr<'src>, ParseError<'src>> {
        let term = self.parse_term()?;
        let mut operations = Vec::new();

        while let Some(op) = self.peek_kind().and_then(BinaryOp::from_token) {
            self.advance()?;
            operations.push((op, self.parse_term()?));
        }

        Ok(Expr {
            term,
            op: operations,
        })
    }

    fn parse_term(&mut self) -> Result<Term<'src>, ParseError<'src>> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::IntLiteral(n) => {
                let value = u16::try_from(n).map_err(|_| ParseError::IntegerOverflow(token))?;
                Ok(Term::IntegerConstant(value))
            }
            TokenKind::StringLiteral(s) => Ok(Term::StringConstant(s)),

            TokenKind::True => Ok(Term::KeywordConstant(KeywordConstant::True)),
            TokenKind::False => Ok(Term::KeywordConstant(KeywordConstant::False)),
            TokenKind::Null => Ok(Term::KeywordConstant(KeywordConstant::Null)),
            TokenKind::This => Ok(Term::KeywordConstant(KeywordConstant::This)),

            TokenKind::Identifier(name) => self.parse_identifier_term(name),

            TokenKind::LParen => {
                let expr = self.parse_expression()?;
                self.expect(&TokenKind::RParen)?;
                Ok(Term::Grouped(Box::new(expr)))
            }

            TokenKind::Minus => Ok(Term::Unary(UnaryOp::Minus, Box::new(self.parse_term()?))),
            TokenKind::Tilde => Ok(Term::Unary(UnaryOp::Tilde, Box::new(self.parse_term()?))),

            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    /// Disambiguates `name`, `name[expr]`, and `name.method()`/`name()`.
    fn parse_identifier_term(&mut self, name: &'src str) -> Result<Term<'src>, ParseError<'src>> {
        if self.peek_is(&TokenKind::LBracket) {
            self.advance()?;
            let index = self.parse_expression()?;
            self.expect(&TokenKind::RBracket)?;
            Ok(Term::ArrayAccess(name, Box::new(index)))
        } else if self.peek_is(&TokenKind::LParen) || self.peek_is(&TokenKind::Dot) {
            Ok(Term::SubroutineCall(self.parse_subroutine_call(name)?))
        } else {
            Ok(Term::Variable(name))
        }
    }

    fn parse_subroutine_call(
        &mut self,
        first: &'src str,
    ) -> Result<SubroutineCall<'src>, ParseError<'src>> {
        let (receiver, name) = if self.eat(&TokenKind::Dot) {
            (Some(first), self.expect_identifier()?)
        } else {
            (None, first)
        };

        self.expect(&TokenKind::LParen)?;
        let args = self.comma_separated(&TokenKind::RParen, Self::parse_expression)?;
        self.expect(&TokenKind::RParen)?;

        Ok(SubroutineCall {
            name,
            receiver,
            args,
        })
    }

    // ── Statement Parsing ────────────────────────────────────────────

    fn parse_block(&mut self) -> Result<Vec<Statement<'src>>, ParseError<'src>> {
        self.expect(&TokenKind::LBrace)?;
        let stmts = self.parse_statements()?;
        self.expect(&TokenKind::RBrace)?;
        Ok(stmts)
    }

    fn parse_statements(&mut self) -> Result<Vec<Statement<'src>>, ParseError<'src>> {
        let mut stmts = Vec::new();
        while !self.is_at_end() && !self.peek_is(&TokenKind::RBrace) {
            stmts.push(self.parse_statement()?);
        }
        Ok(stmts)
    }

    fn parse_statement(&mut self) -> Result<Statement<'src>, ParseError<'src>> {
        let token = self.advance()?;
        match token.kind {
            TokenKind::Let => self.parse_let().map(Statement::Let),
            TokenKind::If => self.parse_if().map(Statement::If),
            TokenKind::While => self.parse_while().map(Statement::While),
            TokenKind::Do => self.parse_do().map(Statement::Do),
            TokenKind::Return => self.parse_return().map(Statement::Return),
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }

    fn parse_let(&mut self) -> Result<LetStmt<'src>, ParseError<'src>> {
        let name = self.expect_identifier()?;

        let index = if self.eat(&TokenKind::LBracket) {
            let idx = self.parse_expression()?;
            self.expect(&TokenKind::RBracket)?;
            Some(idx)
        } else {
            None
        };

        self.expect(&TokenKind::Equal)?;
        let expr = self.parse_expression()?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(LetStmt { name, index, expr })
    }

    fn parse_if(&mut self) -> Result<IfStmt<'src>, ParseError<'src>> {
        self.expect(&TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RParen)?;

        let if_body = self.parse_block()?;
        let else_body = if self.eat(&TokenKind::Else) {
            Some(self.parse_block()?)
        } else {
            None
        };

        Ok(IfStmt {
            condition,
            if_body,
            else_body,
        })
    }

    fn parse_while(&mut self) -> Result<WhileStmt<'src>, ParseError<'src>> {
        self.expect(&TokenKind::LParen)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::RParen)?;
        let body = self.parse_block()?;

        Ok(WhileStmt { condition, body })
    }

    fn parse_do(&mut self) -> Result<DoStmt<'src>, ParseError<'src>> {
        let name = self.expect_identifier()?;
        let subroutine_call = self.parse_subroutine_call(name)?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(DoStmt { subroutine_call })
    }

    fn parse_return(&mut self) -> Result<ReturnStmt<'src>, ParseError<'src>> {
        let expr = if self.peek_is(&TokenKind::Semicolon) {
            None
        } else {
            Some(self.parse_expression()?)
        };
        self.expect(&TokenKind::Semicolon)?;

        Ok(ReturnStmt { expr })
    }

    // ── Declaration Parsing ──────────────────────────────────────────

    fn parse_names(&mut self) -> Result<Vec<&'src str>, ParseError<'src>> {
        let mut names = vec![self.expect_identifier()?];
        while self.eat(&TokenKind::Comma) {
            names.push(self.expect_identifier()?);
        }
        Ok(names)
    }

    fn parse_var_dec(&mut self) -> Result<VarDec<'src>, ParseError<'src>> {
        self.expect(&TokenKind::Var)?;
        let ty = self.parse_type()?;
        let names = self.parse_names()?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(VarDec { names, ty })
    }

    fn parse_class_var_dec(&mut self) -> Result<ClassVarDec<'src>, ParseError<'src>> {
        let token = self.advance()?;
        let kind = match token.kind {
            TokenKind::Static => Kind::Static,
            TokenKind::Field => Kind::Field,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let ty = self.parse_type()?;
        let names = self.parse_names()?;
        self.expect(&TokenKind::Semicolon)?;

        Ok(ClassVarDec { names, kind, ty })
    }

    fn parse_subroutine_dec(&mut self) -> Result<SubroutineDec<'src>, ParseError<'src>> {
        let token = self.advance()?;
        let kind = match token.kind {
            TokenKind::Constructor => SubroutineKind::Constructor,
            TokenKind::Function => SubroutineKind::Function,
            TokenKind::Method => SubroutineKind::Method,
            _ => return Err(ParseError::UnexpectedToken(token)),
        };

        let return_ty = self.parse_return_type()?;
        let name = self.expect_identifier()?;

        self.expect(&TokenKind::LParen)?;
        let parameters = self.comma_separated(&TokenKind::RParen, Self::parse_parameter)?;
        self.expect(&TokenKind::RParen)?;

        self.expect(&TokenKind::LBrace)?;
        let mut variables = Vec::new();
        while self.peek_is(&TokenKind::Var) {
            variables.push(self.parse_var_dec()?);
        }
        let statements = self.parse_statements()?;
        self.expect(&TokenKind::RBrace)?;

        Ok(SubroutineDec {
            kind,
            return_ty,
            name,
            parameters,
            body: SubroutineBody {
                variables,
                statements,
            },
        })
    }

    fn parse_parameter(&mut self) -> Result<Parameter<'src>, ParseError<'src>> {
        let ty = self.parse_type()?;
        let name = self.expect_identifier()?;
        Ok(Parameter { name, ty })
    }

    fn parse_class(&mut self) -> Result<Class<'src>, ParseError<'src>> {
        self.expect(&TokenKind::Class)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::LBrace)?;

        let mut variables = Vec::new();
        while self.peek_matches(|k| matches!(k, TokenKind::Static | TokenKind::Field)) {
            variables.push(self.parse_class_var_dec()?);
        }

        let mut subroutines = Vec::new();
        while self.peek_matches(|k| {
            matches!(
                k,
                TokenKind::Constructor | TokenKind::Function | TokenKind::Method
            )
        }) {
            subroutines.push(self.parse_subroutine_dec()?);
        }

        self.expect(&TokenKind::RBrace)?;

        Ok(Class {
            name,
            variables,
            subroutines,
        })
    }
}
