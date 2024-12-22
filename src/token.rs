use lazy_static::lazy_static;
use std::collections::HashMap;
use std::fmt;

lazy_static! {
    pub static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::AND),
        ("class", TokenType::CLASS),
        ("else", TokenType::ELSE),
        ("false", TokenType::FALSE),
        ("for", TokenType::FOR),
        ("fun", TokenType::FUN),
        ("if", TokenType::IF),
        ("nil", TokenType::NIL),
        ("or", TokenType::OR),
        ("print", TokenType::PRINT),
        ("return", TokenType::RETURN),
        ("super", TokenType::SUPER),
        ("this", TokenType::THIS),
        ("true", TokenType::TRUE),
        ("var", TokenType::VAR),
        ("while", TokenType::WHILE),
    ]);
}

#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,

    EOF,
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone)]
pub struct Token {
    pub ttype: TokenType,
    pub lexeme: String,
    pub literal: Option<LiteralValue>,
}

#[derive(Clone)]
pub enum LiteralValue {
    StringLiteral(String),
    NumberLiteral(f64),
    BoolLiteral(bool),
    NilLiteral,
}

impl Token {
    pub fn new(ttype: TokenType, lexeme: String, literal: Option<LiteralValue>) -> Self {
        Token {
            ttype,
            lexeme,
            literal,
        }
    }
}

impl fmt::Display for LiteralValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            LiteralValue::StringLiteral(s) => s.clone(),
            LiteralValue::NumberLiteral(f) => format!("{:?}", f),
            LiteralValue::BoolLiteral(b) => format!("{:?}", b),
            LiteralValue::NilLiteral => "nil".to_string(),
        };
        write!(f, "{}", s)
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            self.ttype,
            self.lexeme,
            self.literal
                .as_ref()
                .map_or("null".to_string(), |s| format!("{}", s))
        )
    }
}
