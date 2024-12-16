use crate::token::*;
use std::fmt;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token>,
    pub status: ScannerStatus, 
}

pub enum ScannerStatus {
    ScanSuccess,
    UnknowCharErr,
}

#[derive(Debug, Clone)]
enum ScannerError {
    UnknownChar(usize, char),
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            status: ScannerStatus::ScanSuccess
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {},
                Err(e) =>  {
                    eprintln!("{}", e);
                    match e {
                        ScannerError::UnknownChar(_, _) => self.status = ScannerStatus::UnknowCharErr,
                    }
                },
            }
        }
        self.add_token(TokenType::EOF);
    }

    pub fn print_tokens(&self) {
        for token in &self.tokens {
            println!("{}", token);
        }
    }

    fn scan_token(&mut self) -> Result<(), ScannerError> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LEFT_PAREN),
            ')' => self.add_token(TokenType::RIGHT_PAREN),
            '{' => self.add_token(TokenType::LEFT_BRACE),
            '}' => self.add_token(TokenType::RIGHT_BRACE),
            ',' => self.add_token(TokenType::COMMA),
            '.' => self.add_token(TokenType::DOT),
            '-' => self.add_token(TokenType::MINUS),
            '+' => self.add_token(TokenType::PLUS),
            '*' => self.add_token(TokenType::STAR),
            ';' => self.add_token(TokenType::SEMICOLON),
            '\n' => self.line += 1,
            _ => return Err(ScannerError::UnknownChar(self.line, c)),
        };
        Ok(())
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

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownChar(line, c) => write!(f, "[line {}] Error: Unexpected character: {}", line, c)
        }
    }
}
