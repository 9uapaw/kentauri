use crate::scanner::token::Token;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Error {
    token: Option<Token>,
    message: String,
}

impl Error {
    pub fn new(token: Token, message: &str) -> Self {
        Error {
            token: Some(token),
            message: String::from(message),
        }
    }

    pub fn message(message: &str) -> Self {
        Error {
            token: None,
            message: String::from(message),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match &self.token {
            Some(token) => write!(f, "{} {}", token, self.message),
            None => write!(f, "{}", self.message),
        }
    }
}
