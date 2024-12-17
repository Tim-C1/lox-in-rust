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
    NonTerminatedStringErr,
}

#[derive(Debug, Clone)]
enum ScannerError {
    UnknownChar(usize, char),
    NonTerminatedString(usize),
}

impl<'a> Scanner<'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            status: ScannerStatus::ScanSuccess,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.end() {
            self.start = self.current;
            match self.scan_token() {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("{}", e);
                    match e {
                        ScannerError::UnknownChar(_, _) => {
                            self.status = ScannerStatus::UnknowCharErr
                        }
                        ScannerError::NonTerminatedString(_) => {
                            self.status = ScannerStatus::NonTerminatedStringErr
                        }
                    }
                }
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
            '!' => {
                if self.match_then_advance('=') {
                    self.add_token(TokenType::BANG_EQUAL)
                } else {
                    self.add_token(TokenType::BANG)
                }
            }
            '=' => {
                if self.match_then_advance('=') {
                    self.add_token(TokenType::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenType::EQUAL)
                }
            }
            '<' => {
                if self.match_then_advance('=') {
                    self.add_token(TokenType::LESS_EQUAL)
                } else {
                    self.add_token(TokenType::LESS)
                }
            }
            '>' => {
                if self.match_then_advance('=') {
                    self.add_token(TokenType::GREATER_EQUAL)
                } else {
                    self.add_token(TokenType::GREATER)
                }
            }
            '/' => {
                if self.match_then_advance('/') {
                    while self.peek() != '\n' && !self.end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::SLASH)
                }
            }
            ' ' | '\r' | '\t' => {},
            '"' => return self.string(), 
            '\n' => self.line += 1,
            _ => return Err(ScannerError::UnknownChar(self.line, c)),
        };
        Ok(())
    }

    fn advance(&mut self) -> char {
        let c = self.source.as_bytes()[self.current];
        self.current += 1;
        char::from(c)
    }

    fn match_then_advance(&mut self, expected: char) -> bool {
        if self.end() {
            return false;
        }
        if char::from(self.source.as_bytes()[self.current]) != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.end() {
            '\0'
        } else {
            char::from(self.source.as_bytes()[self.current])
        }
    }

    fn string(&mut self) -> Result<(), ScannerError> {
        while self.peek() != '"' && !self.end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.end() {
            Err(ScannerError::NonTerminatedString(self.line))
        } else {
            let literal = String::from(&self.source[self.start+1..self.current]);
            self.add_token_literal(TokenType::STRING, literal);
            Ok(())
        }
    }

    fn end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ttype: TokenType) {
        let lexeme = match ttype {
            TokenType::EOF => String::from(""),
            _ => String::from(&self.source[self.start..self.current]),
        };
        let token = Token::new(ttype, lexeme, None, self.line);
        self.tokens.push(token);
    }

    #[allow(dead_code)]
    fn add_token_literal(&mut self, ttype: TokenType, literal: String) {
        let lexeme = String::from(&self.source[self.start..self.current]);
        let token = Token::new(ttype, lexeme, Some(literal), self.line);
        self.tokens.push(token);
    }
}

impl fmt::Display for ScannerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnknownChar(line, c) => {
                write!(f, "[line {}] Error: Unexpected character: {}", line, c)
            }
            Self::NonTerminatedString(line) => {
                write!(f, "[line {}] Error: Unterminated string.", line)
            }
        }
    }
}
