use crate::bytecode::opcode::OpCode;

#[derive(Debug)]
pub enum Instruction {
    NoParam(OpCode)
}