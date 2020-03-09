use crate::error::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub enum InterpreterError {
    CompilerError(Error),
    SyntaxError(Error),
    RuntimeError(Error),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            InterpreterError::CompilerError(err) => write!(f, "CompilerError: {}", err),
            InterpreterError::SyntaxError(err) => write!(f, "SyntaxError: {}", err),
            InterpreterError::RuntimeError(err) => write!(f, "RuntimeError: {}", err),
        }
    }
}
