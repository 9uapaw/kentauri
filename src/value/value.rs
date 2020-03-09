use crate::error::error;
use crate::value::obj::Obj;
use crate::value::obj_str::ObjStr;
use std::fmt::{Display, Error, Formatter};
use ustr::Ustr;

#[derive(Debug, Clone)]
pub enum Value {
    Bool(bool),
    Nil,
    Number(f64),
    String(ObjStr),
    Object(*mut Obj),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match &self {
            Value::Bool(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s.string.as_str()),
            Value::Object(o) => write!(f, "{:?}", o),
        }
    }
}

impl Value {
    pub fn is_falsy(&self) -> bool {
        let res = match self {
            Value::Nil => true,
            Value::Bool(b) => !*b,
            Value::String(s) => s.string.as_str() == "",
            _ => false,
        };

        res
    }

    pub fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Nil, Value::Nil) => true,
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(l), Value::String(r)) => l.string == r.string,
            (Value::Object(lo), Value::Object(ro)) => lo == ro,
            _ => true,
        }
    }
}

impl From<&mut Obj> for Value {
    fn from(item: &mut Obj) -> Self {
        Value::Object(item as *mut Obj)
    }
}

impl From<&str> for Value {
    fn from(item: &str) -> Self {
        Value::String(ObjStr::new(item))
    }
}

impl Value {
    pub fn deref_obj(&mut self) -> Option<&mut Obj> {
        match self {
            Value::Object(p) => {
                if p.is_null() {
                    return None;
                }
                unsafe { return p.as_mut() }
            }
            _ => None,
        }
    }

    pub fn is_obj(&self) -> bool {
        match self {
            Value::Object(_) => true,
            _ => false,
        }
    }
}

pub struct ValuePool {
    pub values: Vec<Value>,
}

impl ValuePool {
    pub fn new() -> Self {
        ValuePool { values: Vec::new() }
    }
}
