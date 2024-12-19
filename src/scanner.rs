use crate::token::*;
use std::fmt;
use std::str;

pub struct Scanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
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

#[inline]
fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[inline]
pub fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

#[inline]
pub fn is_alpha_numeric(c: char) -> bool {
    is_alpha(c) || is_digit(c)
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
            ' ' | '\r' | '\t' => {}
            '"' => return self.string(),
            c if is_digit(c) => self.number(),
            c if is_alpha(c) => self.identifier(),
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
            return Err(ScannerError::NonTerminatedString(self.line));
        }
        let literal = String::from(&self.source[self.start + 1..self.current]);
        self.advance();
        self.add_token_literal(TokenType::STRING, literal);
        Ok(())
    }

    fn number(&mut self) {
        while is_digit(self.peek()) {
            self.advance();
        }
        if self.peek() == '.' {
            self.advance();
            while is_digit(self.peek()) {
                self.advance();
            }
        }
        let literal = String::from(&self.source[self.start..self.current]);
        self.add_token_literal(TokenType::NUMBER, literal);
    }

    fn identifier(&mut self) {
        while is_alpha_numeric(self.peek()) {
            self.advance();
        }
        let text = &self.source[self.start..self.current];
        let ttype: TokenType = KEYWORDS.get(text).unwrap_or(&TokenType::IDENTIFIER).clone();
        self.add_token(ttype);
    }

    fn end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn add_token(&mut self, ttype: TokenType) {
        let lexeme = match ttype {
            TokenType::EOF => String::from(""),
            _ => String::from(&self.source[self.start..self.current]),
        };
        let token = Token::new(ttype, lexeme, None);
        self.tokens.push(token);
    }

    #[allow(dead_code)]
    fn add_token_literal(&mut self, ttype: TokenType, literal: String) {
        match ttype {
            TokenType::STRING => {
                let lexeme = String::from(&self.source[self.start..self.current]);
                let token = Token::new(
                    ttype,
                    lexeme,
                    Some(Literal::StringLiteral(literal)),
                );
                self.tokens.push(token);
            }
            TokenType::NUMBER => {
                let lexeme = String::from(&self.source[self.start..self.current]);
                let num_literal = Some(Literal::NumberLiteral(str::parse(&literal).unwrap()));
                let token = Token::new(ttype, lexeme, num_literal);
                self.tokens.push(token);
            }
            _ => unimplemented!(),
        }
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
