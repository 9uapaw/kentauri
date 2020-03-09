use crate::scanner::token::TokenType;
use num_enum::TryFromPrimitive;
use std::convert::TryFrom;
#[macro_use]
use lazy_static::lazy_static;
use std::borrow::Borrow;

lazy_static! {
    static ref RULES: Vec<ParseRule> = {
        let v = vec![
          ParseRule::new( Some(ParseFn::Grouping), None,    Precedence::NONE ),       // TOKEN_LEFT_PAREN
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_RIGHT_PAREN
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_LEFT_BRACE
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_RIGHT_BRACE
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_COMMA
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_DOT
          ParseRule::new( Some(ParseFn::Unary),    Some(ParseFn::Binary),  Precedence::TERM ),       // TOKEN_MINUS
          ParseRule::new( None,     Some(ParseFn::Binary),  Precedence::TERM ),       // TOKEN_PLUS
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_SEMICOLON
          ParseRule::new( None,     Some(ParseFn::Binary),  Precedence::FACTOR ),     // TOKEN_SLASH
          ParseRule::new( None,     Some(ParseFn::Binary),  Precedence::FACTOR ),     // TOKEN_STAR
          ParseRule::new( Some(ParseFn::Unary),     None,    Precedence::NONE ),       // TOKEN_BANG
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::EQ ),       // TOKEN_BANG_EQUAL
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_EQUAL
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::EQ ),       // TOKEN_EQUAL_EQUAL
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::COMP),       // TOKEN_GREATER
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::COMP), // TOKEN_GREATER_EQUAL
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::COMP), // TOKEN_LESS
          ParseRule::new( None,     Some(ParseFn::Binary),    Precedence::COMP), // TOKEN_LESS_EQUAL
          ParseRule::new( Some(ParseFn::Variable),     None,    Precedence::NONE ),       // TOKEN_IDENTIFIER
          ParseRule::new( Some(ParseFn::String),     None,    Precedence::NONE ),       // TOKEN_STRING
          ParseRule::new( Some(ParseFn::Number),   None,    Precedence::NONE ),       // TOKEN_NUMBER
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_AND
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_CLASS
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_ELSE
          ParseRule::new( Some(ParseFn::Literal),     None,    Precedence::NONE ),       // TOKEN_FALSE
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_FOR
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_FUN
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_IF
          ParseRule::new( Some(ParseFn::Literal),     None,    Precedence::NONE ),       // TOKEN_NIL
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_OR
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_PRINT
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_RETURN
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_SUPER
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_THIS
          ParseRule::new( Some(ParseFn::Literal),     None,    Precedence::NONE ),       // TOKEN_TRUE
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_VAR
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_WHILE
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_ERROR
          ParseRule::new( None,     None,    Precedence::NONE ),       // TOKEN_EOF
        ];

        v
    };
}

#[derive(TryFromPrimitive, Copy, Clone, Debug)]
#[repr(u8)]
pub enum Precedence {
    NONE = 0,
    ASSIGNMENT,
    OR,
    AND,
    EQ,
    COMP,
    TERM,
    FACTOR,
    UNARY,
    CALL,
    PRIMARY,
}

pub enum ParseFn {
    Binary,
    Unary,
    Number,
    Grouping,
    Literal,
    String,
    Variable,
}

pub struct ParseRule {
    pub prefix: Option<ParseFn>,
    pub infix: Option<ParseFn>,
    pub precedence: Precedence,
}

impl ParseRule {
    pub fn new(prefix: Option<ParseFn>, infix: Option<ParseFn>, precedence: Precedence) -> Self {
        ParseRule {
            prefix,
            infix,
            precedence,
        }
    }

    pub fn get_incremented_prec(&self, add: u8) -> Option<Precedence> {
        let new_prec = Precedence::try_from(self.precedence as u8 + add).ok();

        new_prec
    }
}

pub fn get_rule(op: &TokenType) -> &'static ParseRule {
    RULES.get(*op as usize).unwrap()
}
