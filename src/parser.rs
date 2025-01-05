use crate::expression::*;
use crate::statement::*;
use crate::token::*;
use std::fmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    pub status: ParserStatus,
}

pub struct ParserError {
    token: Token,
    msg: String,
}

pub enum ParserStatus {
    Success,
    Panic,
}

impl ParserError {
    pub fn new(token: Token, msg: &str) -> Self {
        Self {
            token,
            msg: String::from(msg),
        }
    }
}
fn report(line: usize, loc: String, msg: &str) -> String {
    format!("[line {line}] Error {loc}: {msg}")
}
impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            report(
                self.token.line,
                if self.token.ttype == TokenType::EOF {
                    String::from("at end")
                } else {
                    let lexeme = self.token.lexeme.as_str();
                    format!("at '{lexeme}';")
                },
                &self.msg
            )
        )
    }
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            status: ParserStatus::Success,
        }
    }

    pub fn parse_expr(&mut self) -> Result<Box<Expr>, ParserError> {
        self.expression()
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        while !self.end() {
            match self.declaration() {
                Some(stmt) => stmts.push(stmt),
                None => continue,
            }
        }
        stmts
    }

    fn declaration(&mut self) -> Option<Stmt> {
        match if self.match_then_advance(vec![TokenType::VAR]) {
            self.var_declaration()
        } else {
            self.statement()
        } {
            Ok(stmt) => Some(stmt),
            Err(e) => {
                eprintln!("{e}");
                self.status = ParserStatus::Panic;
                self.synchronize();
                None
            }
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParserError> {
        let name = self
            .consume(TokenType::IDENTIFIER, "Expect variable name")?
            .clone();
        let init = if self.match_then_advance(vec![TokenType::EQUAL]) {
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(TokenType::SEMICOLON, "Expect ; after variable declaration")?;
        Ok(Stmt::VarStmt(VarStmtInner(name, init)))
    }

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.match_then_advance(vec![TokenType::IF]) {
            self.if_statement()
        } else if self.match_then_advance(vec![TokenType::PRINT]) {
            self.print_statement()
        } else if self.match_then_advance(vec![TokenType::WHILE]) {
            self.while_statement()
        } else if self.match_then_advance(vec![TokenType::LEFT_BRACE]) {
            self.block_statement()
        } else {
            self.expression_statement()
        }
    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LEFT_PAREN, "expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_then_advance(vec![TokenType::ELSE]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::IfStmt(IfStmtInner::new(
            condition,
            Box::new(then_branch),
            else_branch,
        )))
    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "expect ';' after value.")?;
        Ok(Stmt::PrintStmt(PrintStmtInner(expr)))
    }

    fn while_statement(&mut self) -> Result<Stmt, ParserError> {
        self.consume(TokenType::LEFT_PAREN, "expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RIGHT_PAREN, "expect ')' after condition.")?;
        let body = self.statement()?;
        Ok(Stmt::WhileStmt(WhileStmtInner::new(
            condition,
            Box::new(body),
        )))
    }

    fn block_statement(&mut self) -> Result<Stmt, ParserError> {
        let mut stmts = Vec::new();
        while !self.check(TokenType::RIGHT_BRACE) && !self.end() {
            let stmt = self.declaration();
            if let Some(stmt) = stmt {
                stmts.push(Box::new(stmt));
            }
        }
        self.consume(TokenType::RIGHT_BRACE, "expect '}' after block.")?;
        Ok(Stmt::BlockStmt(BlockStmtInner(stmts)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParserError> {
        let expr = self.expression()?;
        self.consume(TokenType::SEMICOLON, "expect ';' after expression.")?;
        Ok(Stmt::ExprStmt(ExprStmtInner(expr)))
    }

    fn expression(&mut self) -> Result<Box<Expr>, ParserError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Box<Expr>, ParserError> {
        let expr = self.or()?;
        if self.match_then_advance(vec![TokenType::EQUAL]) {
            let value = self.assignment()?;
            match expr.as_ref() {
                Expr::VarExpr(var) => Ok(Box::new(Expr::AssignmentExpr(Assignment::new(
                    var.name.clone(),
                    value,
                )))),
                _ => {
                    let e = ParserError::new(self.previous().clone(), "Invalid assignment target");
                    println!("{e}");
                    Ok(expr)
                }
            }
        } else {
            Ok(expr)
        }
    }

    pub fn or(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.and()?;
        while self.match_then_advance(vec![TokenType::OR]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            left = Box::new(Expr::LogicalExpr(Logical::new(left, operator, right)));
        }
        Ok(left)
    }

    pub fn and(&mut self) -> Result<Box<Expr>, ParserError> {
        let mut left = self.equality()?;
        while self.match_then_advance(vec![TokenType::AND]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            left = Box::new(Expr::LogicalExpr(Logical::new(left, operator, right)));
        }
        Ok(left)
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
        if self.match_then_advance(vec![TokenType::LEFT_PAREN]) {
            let expr = self.expression()?;
            match self.consume(TokenType::RIGHT_PAREN, "expect ')' after expression.") {
                Ok(_) => return Ok(Box::new(Expr::GroupingExpr(Grouping::new(expr)))),
                Err(e) => {
                    return Err(e);
                }
            }
        }
        if self.match_then_advance(vec![TokenType::IDENTIFIER]) {
            return Ok(Box::new(Expr::VarExpr(Var::new(self.previous().clone()))));
        }
        Err(ParserError::new(self.peek().clone(), "exptect expression."))
    }

    fn synchronize(&mut self) {
        self.advance();
        while !self.end() {
            if self.previous().ttype == TokenType::SEMICOLON {
                return;
            }
            match self.peek().ttype {
                TokenType::CLASS
                | TokenType::FUN
                | TokenType::VAR
                | TokenType::FOR
                | TokenType::IF
                | TokenType::WHILE
                | TokenType::PRINT
                | TokenType::RETURN => return,
                _ => {
                    self.advance();
                }
            }
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

    fn consume(&mut self, ttype: TokenType, msg: &str) -> Result<&Token, ParserError> {
        if self.check(ttype) {
            Ok(self.advance())
        } else {
            Err(ParserError::new(self.peek().clone(), msg))
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
