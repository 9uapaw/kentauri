use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCode;
use std::convert::TryFrom;
use crate::util::byte_utils::byte_array_to_u32;

pub fn disassemble_chunk(chunk: &Chunk, name: &str) {
    println!("== {} ==", name);
    
    let mut offset: usize = 0;

    while offset < chunk.code.len() {
       offset = disassemble_instruction(chunk, offset);
    }
}

pub fn disassemble_instruction(chunk: &Chunk, offset: usize) -> usize {
    if offset > 0 && chunk.get_code_line(offset) == chunk.get_code_line(offset - 1) {
        print!("|     {offset:>04} ", offset=offset);
    } else {
        print!("{line}:    {offset:>04} ",line=chunk.get_code_line(offset), offset=offset);
    }

    let op_code = chunk.code.get(offset).unwrap();
    let op_enum = OpCode::try_from(*op_code).expect("Unable to serialize byte to OpCode");

    match op_enum {
        OpCode::OP_RETURN | OpCode::OP_NEGATE | OpCode::OP_ADD | OpCode::OP_SUB | OpCode::OP_MUL
            | OpCode::OP_DIV => simple(op_enum, *op_code, offset),
        OpCode::OP_CONST => constant(*op_code, chunk, offset),
        _ => {print!("UNKNOWN"); offset}
    }


}

fn simple(op: OpCode, op_num: u8, offset: usize) -> usize {
    print!("{:?} {}", op, op_num);
    offset + 1
}

fn constant(op: u8, chunk: &Chunk, offset: usize) -> usize {
    let constant_index = chunk.code.get(offset + 1).unwrap();
    let constant_value = chunk.const_pool.values.get(*constant_index as usize).unwrap();

    print!("OP_CONST {} {:>4} {}", op, constant_index, constant_value);

    offset + 2
}

fn constant_long(op: u8, chunk: &Chunk, offset: usize) -> usize {
    let bytes = chunk.get_byte_sequence(offset + 1, offset + 3);
    let constant_index = byte_array_to_u32(&[0, bytes[0], bytes[1], bytes[2]]);

    let constant_value = chunk.const_pool.values.get(constant_index as usize).unwrap();

    print!("OP_CONST_LONG {} {:>4} {}", op, constant_index, constant_value);

    offset + 4
}

