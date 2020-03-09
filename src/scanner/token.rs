use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexem: String,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexem: String, line: usize) -> Self {
        Token {
            token_type,
            lexem,
            line,
        }
    }

    pub fn eof(line: usize) -> Self {
        Token {
            token_type: TokenType::EOF,
            lexem: String::from(""),
            line,
        }
    }

    pub fn error(message: &str, line: usize) -> Self {
        Token {
            token_type: TokenType::ERROR,
            lexem: String::from(message),
            line: 0,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}: {} <{:?}>", self.line, self.lexem, self.token_type)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Hash, Eq)]
#[repr(u8)]
pub enum TokenType {
    LEFT_PAREN = 0,
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

    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    IDENTIFIER,
    STRING,
    NUMBER,

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

    ERROR,
    EOF,
}
