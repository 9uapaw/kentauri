use kentauri::bytecode::chunk::Chunk;
use kentauri::bytecode::opcode::OpCode;
use kentauri::debug::disassembler::disassemble_chunk;
use kentauri::vm::vm::VM;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write_code(OpCode::OP_CONST, 0);
    chunk.write_const(1.2);
    chunk.write_code(OpCode::OP_CONST, 0);
    chunk.write_const(3.4);
    chunk.write_code(OpCode::OP_ADD, 0);
    chunk.write_code(OpCode::OP_CONST, 0);
    chunk.write_const(5.6);
    chunk.write_code(OpCode::OP_DIV, 0);
    chunk.write_code(OpCode::OP_NEGATE, 0);
    chunk.write_code(OpCode::OP_RETURN, 3);

    let mut vm = VM::new();
    vm.interpret(&chunk);
}
