use crate::expression::*;
use crate::token::*;
use std::fmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub enum ParserError {
    UnmatchedParen
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnmatchedParen => write!(f, "UnmatchedParen detected")
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Box<dyn Expr> {
        self.expression()
    }

    fn expression(&mut self) -> Box<dyn Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expr> {
        let mut expr = self.comparison();
        while self.match_then_advance(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    fn comparison(&mut self) -> Box<dyn Expr> {
        let mut expr = self.term();
        while self.match_then_advance(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    fn term(&mut self) -> Box<dyn Expr> {
        let mut expr = self.factor();
        while self.match_then_advance(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    fn factor(&mut self) -> Box<dyn Expr> {
        let mut expr = self.unary();
        while self.match_then_advance(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Box::new(BinaryExpr::new(expr, operator, right));
        }
        expr
    }

    fn unary(&mut self) -> Box<dyn Expr> {
        if self.match_then_advance(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary();
            Box::new(UnaryExpr::new(operator, right))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Box<dyn Expr> {
        if self.match_then_advance(vec![TokenType::FALSE]) {
            return Box::new(LiteralExpr::new(Literal::BoolLiteral(false)));
        }
        if self.match_then_advance(vec![TokenType::TRUE]) {
            return Box::new(LiteralExpr::new(Literal::BoolLiteral(true)));
        }
        if self.match_then_advance(vec![TokenType::NIL]) {
            return Box::new(LiteralExpr::new(Literal::NilLiteral));
        }
        if self.match_then_advance(vec![TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous().literal.clone().unwrap();
            return Box::new(LiteralExpr::new(literal));
        }
        if self.match_then_advance(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression();
            match self.consume(TokenType::RIGHT_PAREN) {
                Ok(_) => return Box::new(GroupingExpr::new(expr)),
                Err(e) => panic!("{}", e),
            }
        } else {
            unreachable!()
        }
    }

    fn match_then_advance(&mut self, ttypes: Vec<TokenType>) -> bool {
        for t in ttypes {
            if self.check(t) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.end() {
            false
        } else {
            self.tokens[self.current].ttype == ttype
        }
    }

    fn consume(&mut self, ttype: TokenType) -> Result<&Token, ParserError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(ParserError::UnmatchedParen)
        }
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn advance(&mut self) -> &Token {
        if !self.end() {
            self.current += 1;
        }
        self.previous()
    }

    fn end(&self) -> bool {
        self.peek().ttype == TokenType::EOF
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}
