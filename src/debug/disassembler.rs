use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCode;
use crate::util::byte_utils::byte_array_to_u32;
use std::convert::TryFrom;
use std::borrow::Borrow;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);

    let mut offset: usize = 0;

    while offset < chunk.code.len() {
        offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    if offset > 0 && chunk.get_code_line(offset) == chunk.get_code_line(offset - 1) {
        print!("|     {offset:>04} ", offset = offset);
    } else {
        print!(
            "{line}:    {offset:>04} ",
            line = chunk.get_code_line(offset),
            offset = offset
        );
    }

    let op_code = chunk.code.get(offset).unwrap();
    let op_enum = OpCode::try_from(*op_code).expect("Unable to serialize byte to OpCode");

    match op_enum {
        OpCode::OP_RETURN
        | OpCode::OP_NEGATE
        | OpCode::OP_ADD
        | OpCode::OP_SUB
        | OpCode::OP_MUL
        | OpCode::OP_DIV
        | OpCode::OP_NIL
        | OpCode::OP_TRUE
        | OpCode::OP_FALSE
        | OpCode::OP_NOT
        | OpCode::OP_LESS
        | OpCode::OP_GREATER
        | OpCode::OP_EQUAL
        | OpCode::OP_PRINT
        | OpCode::OP_POP => simple(op_enum, *op_code, offset),
        OpCode::OP_CONST
        | OpCode::OP_GET_GLOBAL
        | OpCode::OP_DEF_GLOBAL
        | OpCode::OP_SET_GLOBAL => constant(op_enum, *op_code, chunk, offset),
        OpCode::OP_SET_LOCAL | OpCode::OP_GET_LOCAL => byte_instr(op_enum, *op_code, chunk, offset),
        _ => {
            print!("UNKNOWN {}", *op_code);
            offset
        }
    }
}

fn simple(op: OpCode, op_num: u8, offset: usize) -> usize {
    print!("{:?} {}", op, op_num);
    offset + 1
}

fn constant(op: OpCode, op_num: u8, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code.get(offset + 1).unwrap();
    let constant_value = chunk
        .const_pool
        .values
        .get(*constant_index as usize)
        .unwrap();

    print!(
        "{:?} {} {:>4} {}",
        op, op_num, constant_index, constant_value
    );

    offset + 2
}

fn byte_instr(op: OpCode, op_num: u8, chunk: &Chunk, offset: usize) -> usize {
    let slot = chunk.code.get(offset + 1).unwrap();
    print!("{:?} {} {}", op.borrow(), op_num, slot);

    offset + 2
}

fn constant_long(op: u8, chunk: &Chunk, offset: usize) -> usize {
    let bytes = chunk.get_byte_sequence(offset + 1, offset + 3);
    let constant_index = byte_array_to_u32(&[0, bytes[0], bytes[1], bytes[2]]);

    let constant_value = chunk
        .const_pool
        .values
        .get(constant_index as usize)
        .unwrap();

    print!(
        "OP_CONST_LONG {} {:>4} {}",
        op, constant_index, constant_value
    );

    offset + 4
}
