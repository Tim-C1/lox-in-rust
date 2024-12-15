use crate::token::*;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 0,
            tokens: Vec::new()
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.end() {
            self.start = self.current;
            self.scan_token();
        }
        self.add_token(TokenType::EOF);
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::SEMICOLON),
            '*' => self.add_token(TokenType::STAR),
            '\n' => self.line += 1,
            _ => unimplemented!(),
        }
    }

    fn add_token(&mut self, ttype: TokenType) {
        let lexeme = match ttype {
            TokenType::EOF => String::from(""),
            _ => String::from(&self.source[self.start..self.current]),
        };
        let token = Token::new(ttype, lexeme, None, self.line);
        self.tokens.push(token);
    }

    fn add_token_literal(&mut self, ttype: TokenType, literal: String) {
        let lexeme = String::from(&self.source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, Some(literal), self.line);
        self.tokens.push(token);
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        char::from(c)
    }

    fn end(&self) -> bool {
        self.current >= self.source.len()
    }
}
