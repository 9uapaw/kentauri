use crate::bytecode::chunk::Chunk;
use crate::compiler::compiler::Compiler;
use crate::error::error::Error;
use crate::error::interpreter::InterpreterError;
use crate::vm::vm::VM;
use std::io::Write;
use std::path::Path;
use std::{fs, io};

pub type InterpreterResult<T> = Result<T, InterpreterError>;

pub struct Interpreter {
    vm: VM,
    compiler: Compiler,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            vm: VM::new(),
            compiler: Compiler::new(),
        }
    }

    pub fn run_file(&mut self, path: &str) {
        let content: String = String::from_utf8_lossy(
            &fs::read(Path::new(path)).expect(&format!("Path not found: {}", path)),
        )
        .parse()
        .expect("Unable to parse source file");

        self.interpret(&content);
    }

    pub fn repl(&mut self) {
        let stdin = io::stdin();
        self.compiler = Compiler::new();

        loop {
            let mut buffer = String::new();
            print!("> ");
            io::stdout().flush().unwrap();
            stdin
                .read_line(&mut buffer)
                .expect("Unexpected error on reading input");
            match self.interpret(&buffer) {
                Err(e) => println!("{}", e),
                Ok(_) => (),
            }
        }
    }

    fn interpret(&mut self, source: &str) -> InterpreterResult<()> {
        let mut compilation = self.compiler.compile(source)?;
        println!("{:?}", compilation.chunk.as_ref().unwrap().code);

        let result = self.vm.interpret(compilation.chunk.take().unwrap());

        result
            .map_err(|e| InterpreterError::RuntimeError(e))
            .map(|c| ())
    }
}
