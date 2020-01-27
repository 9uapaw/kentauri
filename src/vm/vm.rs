use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCode;
use crate::value::value::Value;
use crate::debug::disassembler::disassemble_instruction;
use crate::vm::stack::Stack;

#[macro_use]
use crate::debug::debug_print;

const STACK_MAX: usize = 256;

pub enum InterpreterResult {
    Ok,
    CompileError,
    RuntimeError
}

pub struct VM <'a> {
    chunk: Option<&'a Chunk>,
    ip: usize,
    stack: Stack
}

impl<'a> VM<'a> {

    pub fn new() -> Self {
        VM{chunk: None, ip: 0, stack: Stack::new(STACK_MAX)}
    }

    pub fn interpret(&mut self, chunk: &'a Chunk) -> InterpreterResult {
        self.chunk = Some(chunk);
        self.run()
    }

    fn run(&mut self) -> InterpreterResult {

        loop {

            disassemble_instruction(self.chunk.unwrap(), self.ip);
            debug!("   ///   {}", &self.stack);

            let instruction = self.advance_read_instruction();
            match instruction {
                x if x == OpCode::OP_RETURN as u8 => {
                    println!("{}", self.stack.pop());
                    return InterpreterResult::Ok
                },
                x if x == OpCode::OP_CONST as u8 => {
                    let constant = self.advance_read_constant();
                    self.stack.push(constant);
                },
                x if x == OpCode::OP_NEGATE as u8 => {
                    let res = self.stack.pop();
                    self.stack.push(-res);
                }
                x if x == OpCode::OP_ADD as u8 => {
                    self.binary_operation("+")
                },
                x if x == OpCode::OP_SUB as u8 => {
                    self.binary_operation("-")
                },
                x if x == OpCode::OP_MUL as u8 => {
                    self.binary_operation("*")
                },
                x if x == OpCode::OP_DIV as u8 => {
                    self.binary_operation("/")
                },
                _ => return InterpreterResult::RuntimeError
            }
        }

        InterpreterResult::Ok
    }

    fn advance_read_instruction(&mut self) -> u8 {
        let instruction = self.chunk.unwrap().code.get(self.ip).unwrap();
        self.ip += 1;

        *instruction
    }

    fn advance_read_constant(&mut self) -> Value {
        let const_index = self.advance_read_instruction();
        let constant = self.chunk.unwrap().const_pool.values.get(const_index as usize).unwrap();

        *constant
    }

    #[inline]
    fn binary_operation(&mut self, op: &str) {
        let b = self.stack.pop();
        let a = self.stack.pop();

        match op {
            "+" => self.stack.push(a + b),
            "-" => self.stack.push(a - b),
            "*" => self.stack.push(a * b),
            "/" => self.stack.push(a / b),
            _ => panic!("Invalid binary operation")
        }
    }
}