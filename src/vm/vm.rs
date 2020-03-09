use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCode;
use crate::debug::disassembler::disassemble_instruction;
use crate::value::value::Value;
use crate::vm::stack::Stack;
#[macro_use]
use crate::debug::debug_print;
use crate::error::error::Error;
use crate::value::obj::Obj;
use crate::value::obj_str::ObjStr;
use std::collections::HashMap;
use ustr::UstrMap;

const STACK_MAX: usize = 256;

pub type VMRunResult<T> = Result<T, Error>;

pub struct VM {
    chunk: Option<Chunk>,
    ip: usize,
    stack: Stack,
    globals: UstrMap<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            chunk: None,
            ip: 0,
            stack: Stack::new(STACK_MAX),
            globals: UstrMap::default(),
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> VMRunResult<Chunk> {
        self.stack.reset();
        self.ip = 0;

        self.chunk = Some(chunk);
        self.run().map(|o| self.chunk.take().unwrap())
    }

    fn run(&mut self) -> VMRunResult<()> {
        loop {
            disassemble_instruction(self.chunk.as_ref().unwrap(), self.ip);
            debug!("   ///   {}", &self.stack);

            let instruction = self.advance_read_instruction();
            match instruction {
                x if x == OpCode::OP_RETURN as u8 => return Ok(()),
                x if x == OpCode::OP_CONST as u8 => {
                    let constant = self.advance_read_constant();
                    self.stack.push(constant);
                }
                x if x == OpCode::OP_NEGATE as u8 => {
                    let res = self.stack.pop();
                    let val = match res {
                        Value::Number(v) => -v,
                        _ => return Err(self.runtime_error("Operand must be a number")),
                    };
                    self.stack.push(Value::Number(val));
                }
                x if x == OpCode::OP_ADD as u8 => self.binary_operation("+")?,
                x if x == OpCode::OP_SUB as u8 => self.binary_operation("-")?,
                x if x == OpCode::OP_MUL as u8 => self.binary_operation("*")?,
                x if x == OpCode::OP_DIV as u8 => self.binary_operation("/")?,
                x if x == OpCode::OP_FALSE as u8 => self.stack.push(Value::Bool(false)),
                x if x == OpCode::OP_TRUE as u8 => self.stack.push(Value::Bool(true)),
                x if x == OpCode::OP_NIL as u8 => self.stack.push(Value::Nil),
                x if x == OpCode::OP_NOT as u8 => {
                    let val = self.stack.pop();
                    self.stack.push(Value::Bool(val.is_falsy()))
                }
                x if x == OpCode::OP_EQUAL as u8 => {
                    let b = self.stack.pop();
                    let a = self.stack.pop();

                    self.stack.push(Value::Bool(a.eq(&b)))
                }
                x if x == OpCode::OP_GREATER as u8 => {
                    self.binary_operation(">");
                }
                x if x == OpCode::OP_LESS as u8 => {
                    self.binary_operation(">");
                }
                x if x == OpCode::OP_PRINT as u8 => {
                    let val = self.stack.pop();
                    println!("{}", val);
                }
                x if x == OpCode::OP_POP as u8 => {
                    self.stack.pop();
                }
                x if x == OpCode::OP_DEF_GLOBAL as u8 => {
                    let const_op = self.advance_read_constant();
                    let val = match const_op {
                        Value::String(s) => s.string.clone(),
                        _ => {
                            self.runtime_error("Invalid variable name type.");
                            continue;
                        }
                    };

                    let stack_val = self.stack.pop();

                    self.globals.insert(val, stack_val);
                }
                x if x == OpCode::OP_GET_GLOBAL as u8 => {
                    let identifier = self.advance_read_constant();
                    let val = match identifier {
                        Value::String(s) => s.string.clone(),
                        _ => {
                            self.runtime_error("Invalid variable name type.");
                            continue;
                        }
                    };
                    if let Some(v) = self.globals.get(&val) {
                        self.stack.push(v.clone());
                    } else {
                        return Err(self.runtime_error(&format!("Undefined variable '{}'.", val.as_str())));
                    }
                },
                x if x == OpCode::OP_SET_GLOBAL as u8 => {
                    let const_op = self.advance_read_constant();
                    let val = match const_op {
                        Value::String(s) => s.string.clone(),
                        _ => {
                            return Err(self.runtime_error("Invalid variable name type."));

                        }
                    };
                    if !self.globals.contains_key(&val) {
                        return Err(self.runtime_error(&format!("Undefined variable '{}'.", val.as_str())));
                    } else {
                        let stack_val = self.stack.peek();

                        self.globals.insert(val, stack_val);
                    }

                },
                x if x == OpCode::OP_GET_LOCAL as u8 => {
                    let slot = self.advance_read_instruction();
                    let val = self.stack.get(slot as usize);
                    self.stack.push(val);
                },
                x if x == OpCode::OP_SET_LOCAL as u8 => {
                    let slot = self.advance_read_instruction();
                    let last_val = self.stack.peek();
                    self.stack.set(last_val, slot as usize);
                }
                _ => return Err(Error::message("Unknown opcode")),
            }
        }

        Ok(())
    }

    #[inline]
    fn advance_read_instruction(&mut self) -> u8 {
        let instruction = self.chunk.as_ref().unwrap().code.get(self.ip).unwrap();
        self.ip += 1;

        *instruction
    }

    #[inline]
    fn advance_read_constant(&mut self) -> Value {
        let const_index = self.advance_read_instruction();
        let constant = self
            .chunk
            .as_ref()
            .unwrap()
            .const_pool
            .values
            .get(const_index as usize)
            .unwrap();

        constant.clone()
    }

    fn read_current_executed_line(&mut self) -> usize {
        self.chunk.as_ref().unwrap().get_code_line(self.ip)
    }

    #[inline]
    fn binary_operation(&mut self, op: &str) -> VMRunResult<()> {
        let b = self.stack.pop();
        let a = self.stack.pop();

        match (op, a, b) {
            ("+", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Number(l + r)),
            ("+", Value::String(l), Value::String(r)) => self.stack.push(Value::from(
                (String::from(l.string.as_str()) + r.string.as_str()).as_str(),
            )),
            ("-", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Number(l - r)),
            ("*", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Number(l * r)),
            ("*", Value::String(l), Value::Number(r)) => self
                .stack
                .push(Value::from((l.string.as_str().repeat(r as usize)).as_str())),
            ("/", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Number(l / r)),
            (">", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Bool(l > r)),
            ("<", Value::Number(l), Value::Number(r)) => self.stack.push(Value::Bool(l < r)),
            _ => return Err(self.runtime_error("Invalid binary operator")),
        }

        Ok(())
    }

    fn runtime_error(&mut self, message: &str) -> Error {
        Error::message(&format!(
            "{}: {}",
            self.read_current_executed_line(),
            message
        ))
    }
}
