use crate::expr::Expr;
use crate::lox::Lox;
use crate::token::{LiteralValue, Token, TokenType};
use crate::stmt::Stmt; 

struct ParseError;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self, lox: &mut Lox) -> Option<Vec<Stmt>> {
        let mut statements = Vec::new(); 

        while !self.is_at_end() {
            if let Some(stmt) = self.declaration(lox) {
                statements.push(stmt);
            }
        }
        Some(statements)
    }

    fn declaration(&mut self, lox: &mut Lox) -> Option<Stmt> {
        match self.try_declaration(lox) {
            Ok(stmt) => Some(stmt), 
            Err(ParseError) => {
                self.synchronize();
                None
            }
        }

    }

    fn try_declaration(&mut self, lox:&mut Lox) -> Result<Stmt, ParseError> {
        if self.match_types(&[TokenType::Var]) {
            return self.var_declaration(lox); 
        }
        self.statement(lox)
    }

    fn var_declaration(&mut self, lox:&mut Lox) -> Result<Stmt, ParseError> {
        let name = self.consume(lox, TokenType::Identifier, "Expect variable name.")?.clone(); 
        let initializer = if self.match_types(&[TokenType::Equal]) {
            Some(Box::new(self.expression(lox)?))
        } else {
            None
        };

        self.consume(lox, TokenType::Semicolon, "Expect ';' after variable declaration.")?;
        Ok(Stmt::Var { name, initializer })
    }

    fn expression(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        self.assignment(lox)
    }

    fn statement(&mut self, lox: &mut Lox) -> Result<Stmt, ParseError> {
        if self.match_types(&[TokenType::Print]) {
            return self.print_statement(lox); 
        }
        self.expression_statement(lox)
    }

    fn print_statement(&mut self, lox: &mut Lox) -> Result<Stmt, ParseError> {
        let value = self.expression(lox)?;
        self.consume(lox, TokenType::Semicolon, "Expect ';' after value.")?; 
        Ok(Stmt::Print {
            expression: Box::new(value),
        })
    }

    fn expression_statement(&mut self, lox: &mut Lox) -> Result<Stmt, ParseError> {
        let expr = self.expression(lox)?; 
        self.consume(lox, TokenType::Semicolon, "Expect ';' after expression.")?;
        
        Ok(Stmt::Expression {
            expression: Box::new(expr),
        })
    }

    fn assignment(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        // parse left side expression, then check if it is assignable
        let expr = self.equality(lox)?;

        // consime if next token is =
        if self.match_types(&[TokenType::Equal]) {
            let equals = self.previous().clone(); 

            // recursively call assignment to parse right side
            let value = self.assignment(lox)?; 

            if let Expr::Variable { name } = expr {
                return Ok(Expr::Assign {
                    name, 
                    value: Box::new(value),
                })
            }

            Self::error(lox, &equals, "Invalid assignment target.");
        }

        Ok(expr)
    }



    fn equality(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        let mut expr = self.comparison(lox)?;

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison(lox)?;
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn match_types(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t.clone()) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: TokenType) -> bool {
        !self.is_at_end() && self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn comparison(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        let mut expr = self.term(lox)?;

        while self.match_types(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous().clone(); 
            let right = self.term(lox)?; 

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        let mut expr = self.factor(lox)?;
        
        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone(); 
            let right = self.factor(lox)?; 

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        let mut expr = self.unary(lox)?;
        
        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone(); 
            let right = self.unary(lox)?; 

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary(lox)?; 
            let expr = Expr::Unary {
                operator,
                right: Box::new(right),
            };
            return Ok(expr);
        }
        Ok(self.primary(lox)?)
    }

    fn primary(&mut self, lox: &mut Lox) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::False]) {
            let expr = Expr::Literal {
                value: LiteralValue::Boolean(false)
            };

            return Ok(expr);
        }

        if self.match_types(&[TokenType::True]) {
            let expr = Expr::Literal {
                value: LiteralValue::Boolean(true)
            };

            return Ok(expr);
        }

        if self.match_types(&[TokenType::Nil]) {
            let expr = Expr::Literal {
                value: LiteralValue::Nil
            };

            return Ok(expr);
        }

        if self.match_types(&[TokenType::Number, TokenType::String]) {
            let value = self.previous().literal.clone().expect("scanner should set literal for NUMBER/STRING");
            return Ok(Expr::Literal {value} );
        }

        if self.match_types(&[TokenType::Identifier]) {
            return Ok(Expr::Variable {
                name: self.previous().clone(),
            });
        }

        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.expression(lox)?;
            self.consume(lox, TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping { expression: Box::new(expr) });
        }

        Err(Self::error(lox, self.peek(), "Expect expression."))
    }

    fn consume(
        &mut self,
        lox: &mut Lox,
        kind: TokenType,
        message: &str,
    ) -> Result<&Token, ParseError> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(Self::error(lox, self.peek(), message))
        }
    }

    fn error(lox: &mut Lox, token: &Token, message: &str) -> ParseError {
        lox.error(token, message);
        ParseError
    }
    
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenType::Semicolon {
                return;
            }

            match self.peek().kind {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }
}