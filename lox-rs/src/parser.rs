use crate::{token::{Token, TokenType, LoxType}, ast::{Expr, LoxLiteral}, error::LoxError};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Expr, LoxError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, LoxError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = match self.comparison() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = match self.comparison() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = match self.term() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matches(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {
            let operator = self.previous();
            let right = match self.term() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = match self.factor() {
            Ok(expr) => expr,
            Err(e) => return Err(e)
        };

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = match self.factor() {
                Ok(expr) => expr,
                Err(e) => return Err(e),
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = match self.unary() {
            Ok(expr) => expr,
            Err(e) => return Err(e),
        };

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) }
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };
            return Ok(Expr::Unary { operator, right: Box::new(right) })
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.matches(&[TokenType::False]) {
            return Ok(Expr::Literal { value: LoxLiteral::Bool(false) })
        }
        if self.matches(&[TokenType::True]) {
            return Ok(Expr::Literal { value: LoxLiteral::Bool(true) })
        }
        if self.matches(&[TokenType::Nil]) {
            return Ok(Expr::Literal { value: LoxLiteral::Nil })
        }

        if self.matches(&[TokenType::String, TokenType::Number]) {
            let literal = self.previous().literal;
            if let LoxType::String(str) = literal {
                return Ok(Expr::Literal { value: LoxLiteral::String(str) })
            }
            if let LoxType::Number(num) = literal {
                return Ok(Expr::Literal { value: LoxLiteral::Number(num) })
            }
        }

        if self.matches(&[TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(expr) => expr,
                Err(e) => return Err(e)
            };
            if let Err(e) = self.consume(&TokenType::RightParen, "Expect ')' after expression.") {
                return Err(e);
            };
            return Ok(Expr::Grouping { expression: Box::new(expr)});
        }
        
        Err(LoxError::ParseError { msg: "Expect expression.".into(), line: self.peek().line })
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().r#type == TokenType::Semicolon {
                return;
            }

            match self.peek().r#type {
                TokenType::Class  |
                TokenType::Fun    |
                TokenType::Var    |
                TokenType::For    |
                TokenType::If     |
                TokenType::While  |
                TokenType::Print  |
                TokenType::Return  => return,
                _ => {
                    self.advance();
                }
            }
        }
    }
}

impl Parser {
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for t in types {
            if self.check(t) {
                self.advance();
                return true
            }
        }

        false
    }

    fn check(&self, t: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().r#type == *t
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().r#type == TokenType::Eof
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume<'a>(&'a mut self, t: &TokenType, msg: &'a str) -> Result<Token, LoxError> {
        if self.check(t) {
            return Ok(self.advance());
        }
        Err(LoxError::ParseError { msg: msg.into(), line: self.peek().line })
    }
}