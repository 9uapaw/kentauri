use kentauri::bytecode::chunk::Chunk;
use kentauri::bytecode::opcode::OpCode;
use kentauri::debug::disassembler::disassemble_chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_code(OpCode::OP_RETURN, 0);
    chunk.write_code(OpCode::OP_CONST, 0);
    let const_pos = chunk.add_const(1.2) as u8;
    chunk.write_byte(const_pos, 0);
    chunk.write_code(OpCode::OP_CONST, 3);
    let const_pos = chunk.add_const(1.2) as u8;
    chunk.write_byte(const_pos, 3);


    disassemble_chunk(&chunk, "test_chunk");

}
