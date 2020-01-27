use crate::bytecode::opcode::OpCode;
use crate::value::value::{ValuePool, Value};
use crate::bytecode::instruction::Instruction;


pub struct Chunk {
    pub code: Vec<u8>,
    pub const_pool: ValuePool,
    line_code_index: Vec<usize>,
}

impl Chunk {

    pub fn new() -> Self {
        Chunk{code: Vec::new(), const_pool: ValuePool::new(), line_code_index: Vec::new()}
    }

    pub fn write_code(&mut self, code: OpCode, line: usize) {
        self.write_byte(code as u8, line);

    }

    pub fn write_byte(&mut self, byte: u8, line: usize) {
        self.code.push(byte);

        if self.line_code_index.len() == line {
            self.line_code_index.push(1);
        } else if line > self.line_code_index.len() && line - self.line_code_index.len() >= 1 {
            let start = self.line_code_index.len();
            let end = line;

            for i in start..end {
                self.line_code_index.push(0);
            }

            self.line_code_index.push(1);
        } else {
            self.line_code_index[line] += 1;
        }

    }

    pub fn write_const(&mut self, value: Value) {
        let i = self.add_const(value);
        self.write_byte(i as u8, 0);
    }

    pub fn add_const(&mut self, value: Value) -> usize {
        let pos = if self.const_pool.values.is_empty() {0} else {self.const_pool.values.len()};

        self.const_pool.values.push(value);

        pos
    }

    pub fn get_byte_sequence(&self, start: usize, last: usize) -> &[u8] {
        &self.code[start..=last]
    }

    pub fn get_code_line(&self, offset: usize) -> usize {
        let mut line = 0;
        let mut current_offset = 0;

        while line != self.line_code_index.len() - 1 && offset > current_offset {
            line += 1;
            current_offset += self.line_code_index[line];
        }

        line
    }
}
