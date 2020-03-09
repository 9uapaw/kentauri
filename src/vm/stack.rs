use crate::value::value::Value;
use std::fmt::{Display, Error, Formatter};

#[derive(Debug)]
pub struct Stack {
    stack: Vec<Value>,
    stack_top: usize,
}

impl Stack {
    pub fn new(capacity: usize) -> Self {
        Stack {
            stack: Vec::with_capacity(capacity),
            stack_top: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, v: Value) {
        self.stack.push(v);
        self.stack_top += 1
    }

    #[inline]
    pub fn pop(&mut self) -> Value {
        self.stack_top += 1;
        self.stack.pop().unwrap()
    }

    #[inline]
    pub fn peek(&self) -> Value {
        self.stack.last().unwrap().clone()
    }

    #[inline]
    pub fn get(&self, i: usize) -> Value {
        self.stack.get(i).unwrap().clone()
    }

    #[inline]
    pub fn set(&mut self, v: Value, i: usize) {
        self.stack[i] = v;
    }

    pub fn try_pop(&mut self) -> Option<Value> {
        self.stack_top += 1;
        self.stack.pop()
    }

    pub fn reset(&mut self) {
        self.stack_top = 0;
    }
}

impl Display for Stack {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "STACK: {:?}", self.stack);

        Ok(())
    }
}
