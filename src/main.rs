use kentauri::bytecode::chunk::Chunk;
use kentauri::bytecode::opcode::OpCode;
use kentauri::debug::disassembler::disassemble_chunk;
use kentauri::interpreter::interpreter::Interpreter;
use kentauri::vm::vm::VM;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter = Interpreter::new();

    if args.len() > 2 {
        println!("Usage: <path>");
        exit(64);
    } else if args.len() == 2 {
        interpreter.run_file(args.get(1).unwrap())
    } else {
        interpreter.repl();
    }
}
