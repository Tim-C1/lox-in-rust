use crate::expression::*;
use crate::token::*;
use std::fmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

pub enum ParserError {
    UnmatchedParen,
    ExpectExpr,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnmatchedParen => write!(f, "UnmatchedParen detected"),
            Self::ExpectExpr => write!(f, "Expect expression"),
        }
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Result<Box<Expr>, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.comparison()?;
        while self.match_then_advance(vec![TokenType::BANG_EQUAL, TokenType::EQUAL_EQUAL]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::BinaryExpr(Binary::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.term()?;
        while self.match_then_advance(vec![
            TokenType::GREATER,
            TokenType::GREATER_EQUAL,
            TokenType::LESS,
            TokenType::LESS_EQUAL,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::BinaryExpr(Binary::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.factor()?;
        while self.match_then_advance(vec![TokenType::MINUS, TokenType::PLUS]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::BinaryExpr(Binary::new(expr, operator, right)));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut expr = self.unary()?;
        while self.match_then_advance(vec![TokenType::SLASH, TokenType::STAR]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::BinaryExpr(Binary::new(expr, operator, right)))
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.match_then_advance(vec![TokenType::BANG, TokenType::MINUS]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            Ok(Box::new(Expr::UnaryExpr(Unary::new(operator, right))))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Box<Expr>, ParserError> {
        if self.match_then_advance(vec![TokenType::FALSE]) {
            return Ok(Box::new(Expr::LiteralExpr(Literal::new(
                LiteralValue::BoolLiteral(false),
            ))));
        }
        if self.match_then_advance(vec![TokenType::TRUE]) {
            return Ok(Box::new(Expr::LiteralExpr(Literal::new(
                LiteralValue::BoolLiteral(true),
            ))));
        }
        if self.match_then_advance(vec![TokenType::NIL]) {
            return Ok(Box::new(Expr::LiteralExpr(Literal::new(
                LiteralValue::NilLiteral,
            ))));
        }
        if self.match_then_advance(vec![TokenType::NUMBER, TokenType::STRING]) {
            let literal = self.previous().literal.clone().unwrap();
            return Ok(Box::new(Expr::LiteralExpr(Literal::new(literal))));
        }
        if self.match_then_advance(vec![TokenType::IDENTIFIER]) {
            let literal = self.previous().lexeme.clone();
            return Ok(Box::new(Expr::LiteralExpr(Literal::new(
                LiteralValue::StringLiteral(literal),
            ))));
        }
        if self.match_then_advance(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            match self.consume(TokenType::RIGHT_PAREN) {
                Ok(_) => return Ok(Box::new(Expr::GroupingExpr(Grouping::new(expr)))),
                Err(e) => {
                    eprintln!("{e}");
                    Err(e)
                }
            }
        } else {
            let e = ParserError::UnmatchedParen;
            eprintln!("{e}");
            Err(e)
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
