use crate::parse::*;
use crate::token::{Keyword, Symbol, Token, TokenKind};

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

    // --- Class Parsing ---
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

    // --- Subroutine Parsing ---

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
        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;

        // Parse var declarations (must come first)
        let mut variables: Vec<VarDec> = Vec::new();
        while self.peek().map(|t| t.kind) == Some(TokenKind::Keyword(Keyword::Var)) {
            self.advance(); // consume 'var'
            let kind = self.parse_kind()?;

            let mut names = vec![self.expect_identifier()?];
            while self.peek().map(|t| t.kind) == Some(TokenKind::Symbol(Symbol::Comma)) {
                self.advance(); // consume ','
                names.push(self.expect_identifier()?);
            }

            self.expect(TokenKind::Symbol(Symbol::Semicolon))?;
            variables.push(VarDec { kind, names });
        }

        // Parse statements
        let mut statements: Vec<Statement> = Vec::new();
        while self.peek().map(|t| t.kind) != Some(TokenKind::Symbol(Symbol::RightBrace)) {
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

    // --- Statement Parsing ---

    pub fn parse_statement(&mut self) -> Result<Statement<'t>, ParseError<'t>> {
        match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Let) => {
                    let name = self.expect_identifier()?;
                    let (name, index) = if self.peek().map(|token| token.kind)
                        == Some(TokenKind::Symbol(Symbol::LeftBracket))
                    {
                        self.advance(); // consume '['
                        let index = self.parse_expression()?;
                        self.expect(TokenKind::Symbol(Symbol::RightBracket))?; // consume ']'
                        (name, Some(index))
                    } else {
                        (name, None)
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

                    // If body
                    self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;
                    let mut statements: Vec<Statement> = Vec::new();
                    while self.peek().map(|t| t.kind) != Some(TokenKind::Symbol(Symbol::RightBrace))
                    {
                        statements.push(self.parse_statement()?);
                    }
                    self.expect(TokenKind::Symbol(Symbol::RightBrace))?;

                    // Optional else body
                    let else_statements = if self.peek().map(|token| token.kind)
                        == Some(TokenKind::Keyword(Keyword::Else))
                    {
                        self.advance();
                        self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;
                        let mut statements: Vec<Statement> = Vec::new();
                        while self.peek().map(|t| t.kind)
                            != Some(TokenKind::Symbol(Symbol::RightBrace))
                        {
                            statements.push(self.parse_statement()?);
                        }
                        self.expect(TokenKind::Symbol(Symbol::RightBrace))?;
                        Some(statements)
                    } else {
                        None
                    };

                    Ok(Statement::If(IfStatement::new(
                        condition,
                        statements,
                        else_statements,
                    )))
                }

                TokenKind::Keyword(Keyword::While) => {
                    self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
                    let condition = self.parse_expression()?;
                    self.expect(TokenKind::Symbol(Symbol::RightParen))?;

                    self.expect(TokenKind::Symbol(Symbol::LeftBrace))?;
                    let mut statements: Vec<Statement> = Vec::new();
                    while self.peek().map(|t| t.kind) != Some(TokenKind::Symbol(Symbol::RightBrace))
                    {
                        statements.push(self.parse_statement()?);
                    }
                    self.expect(TokenKind::Symbol(Symbol::RightBrace))?;

                    Ok(Statement::While(WhileStatement::new(condition, statements)))
                }

                TokenKind::Keyword(Keyword::Do) => {
                    let subroutine_name = self.expect_identifier()?;

                    let (receiver, name) =
                        if self.peek().map(|t| t.kind) == Some(TokenKind::Symbol(Symbol::Dot)) {
                            self.advance(); // consume '.'
                            let method = self.expect_identifier()?;
                            (Some(subroutine_name), method)
                        } else {
                            (None, subroutine_name)
                        };

                    self.expect(TokenKind::Symbol(Symbol::LeftParen))?;

                    let mut arguments: Vec<Expression> = Vec::new();
                    while self.peek().map(|t| t.kind) != Some(TokenKind::Symbol(Symbol::RightParen))
                    {
                        arguments.push(self.parse_expression()?);
                        if self.peek().map(|t| t.kind) == Some(TokenKind::Symbol(Symbol::Comma)) {
                            self.advance();
                        } else {
                            break;
                        }
                    }

                    self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                    self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

                    Ok(Statement::Do(DoStatement::new(SubroutineCall {
                        name,
                        receiver,
                        arguments,
                    })))
                }

                TokenKind::Keyword(Keyword::Return) => {
                    // Fix 2: peek before parsing — `return;` is valid in Jack (void return)
                    let expression = if self.peek().map(|t| t.kind)
                        != Some(TokenKind::Symbol(Symbol::Semicolon))
                    {
                        Some(self.parse_expression()?)
                    } else {
                        None
                    };
                    self.expect(TokenKind::Symbol(Symbol::Semicolon))?;

                    Ok(Statement::Return(ReturnStatement::new(expression)))
                }

                _ => Err(ParseError::UnexpectedToken(token)),
            },

            None => Err(ParseError::UnexpectedEof),
        }
    }

    // --- Expression Parsing ---
    fn parse_expression(&mut self) -> Result<Expression<'t>, ParseError<'t>> {
        let term = self.parse_term()?;
        // Fix 3: operations is now Vec, not Option<Vec>
        let mut operations = Vec::new();

        while let Some(operation) = self.peek_binary_operation() {
            self.advance(); // consume operator
            let term = self.parse_term()?;
            operations.push((operation, term));
        }

        Ok(Expression { term, operations })
    }

    // --- Operation Parsing ---

    fn peek_binary_operation(&self) -> Option<Operation> {
        match self.peek()?.kind {
            TokenKind::Symbol(Symbol::Plus) => Some(Operation::Add),
            TokenKind::Symbol(Symbol::Minus) => Some(Operation::Sub),
            TokenKind::Symbol(Symbol::Star) => Some(Operation::Mul),
            TokenKind::Symbol(Symbol::Slash) => Some(Operation::Div),
            TokenKind::Symbol(Symbol::Ampersand) => Some(Operation::And),
            TokenKind::Symbol(Symbol::Pipe) => Some(Operation::Or),
            TokenKind::Symbol(Symbol::GreaterThan) => Some(Operation::GreaterThan),
            TokenKind::Symbol(Symbol::LessThan) => Some(Operation::LessThan),
            TokenKind::Symbol(Symbol::Equal) => Some(Operation::Equal),
            _ => None,
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Expression<'t>>, ParseError<'t>> {
        let mut arguments = Vec::new();
        while self.peek().map(|t| t.kind) != Some(TokenKind::Symbol(Symbol::RightParen)) {
            arguments.push(self.parse_expression()?);
            if self.peek().map(|t| t.kind) == Some(TokenKind::Symbol(Symbol::Comma)) {
                self.advance();
            } else {
                break;
            }
        }
        Ok(arguments)
    }

    // --- Term Parsing ---
    fn parse_term(&mut self) -> Result<Term<'t>, ParseError<'t>> {
        match self.advance() {
            Some(token) => match token.kind {
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
                    match self.peek().map(|t| t.kind) {
                        Some(TokenKind::Symbol(Symbol::LeftBracket)) => {
                            self.advance(); // consume '['
                            let index = self.parse_expression()?;
                            self.expect(TokenKind::Symbol(Symbol::RightBracket))?;
                            Ok(Term::ArrayAccess(name, Box::new(index)))
                        }
                        Some(TokenKind::Symbol(Symbol::Dot)) => {
                            self.advance(); // consume '.'
                            let method = self.expect_identifier()?;
                            self.expect(TokenKind::Symbol(Symbol::LeftParen))?;
                            let arguments = self.parse_arguments()?;
                            self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                            Ok(Term::SubroutineCall(SubroutineCall {
                                receiver: Some(name),
                                name: method,
                                arguments,
                            }))
                        }
                        Some(TokenKind::Symbol(Symbol::LeftParen)) => {
                            self.advance(); // consume '('
                            let arguments = self.parse_arguments()?;
                            self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                            Ok(Term::SubroutineCall(SubroutineCall {
                                receiver: None,
                                name,
                                arguments,
                            }))
                        }
                        _ => Ok(Term::Variable(name)),
                    }
                }
                TokenKind::Symbol(Symbol::LeftParen) => {
                    let expr = self.parse_expression()?;
                    self.expect(TokenKind::Symbol(Symbol::RightParen))?;
                    Ok(Term::Grouped(Box::new(expr)))
                }
                TokenKind::Symbol(Symbol::Minus) => {
                    let term = self.parse_term()?;
                    Ok(Term::Unary(UnaryOperation::Minus, Box::new(term)))
                }
                TokenKind::Symbol(Symbol::Tilde) => {
                    let term = self.parse_term()?;
                    Ok(Term::Unary(UnaryOperation::Tilde, Box::new(term)))
                }
                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    // --- Type Parsing ---

    #[rustfmt::skip]
    fn parse_kind(&mut self) -> Result<Kind<'t>, ParseError<'t>> {
        match self.advance() {
            Some(token) => match token.kind {
                TokenKind::Keyword(Keyword::Int)     => Ok(Kind::Int),
                TokenKind::Keyword(Keyword::Char)    => Ok(Kind::Char),
                TokenKind::Keyword(Keyword::Boolean) => Ok(Kind::Boolean),
                // Fix 1: handle class types e.g. `field Point p;`
                TokenKind::Identifier(name)          => Ok(Kind::Class(name)),

                _ => Err(ParseError::UnexpectedToken(token)),
            },
            None => Err(ParseError::UnexpectedEof),
        }
    }

    // --- Token Navigation Helpers ---

    fn peek(&self) -> Option<Token<'t>> {
        self.tokens.get(self.pos).copied()
    }

    fn advance(&mut self) -> Option<Token<'t>> {
        if self.pos < self.tokens.len() {
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
