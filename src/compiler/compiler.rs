use crate::bytecode::chunk::Chunk;
use crate::bytecode::opcode::OpCode;
use crate::bytecode::opcode::OpCode::OP_CONST;
use crate::compiler::compilation::Compilation;
use crate::compiler::precedence::{get_rule, ParseFn, ParseRule, Precedence};
use crate::compiler::scope::ScopeTracker;
use crate::debug::disassembler::disassemble_chunk;
use crate::error::error::Error;
use crate::error::interpreter::InterpreterError;
use crate::interpreter::interpreter::InterpreterResult;
use crate::scanner::scanner::Scanner;
use crate::scanner::token::{Token, TokenType};
use crate::value::obj::Obj;
use crate::value::obj_str::ObjStr;
use crate::value::value::Value;
use std::collections::HashSet;

pub struct Compiler {
    current: Option<Token>,
    previous: Option<Token>,
    scanner: Option<Scanner>,
    chunk: Option<Chunk>,
    errors: Vec<InterpreterError>,
    panic: bool,
    compilation: Option<Compilation>,
    scope: ScopeTracker,
}

impl Compiler {
    pub fn new() -> Self {
        Compiler {
            current: None,
            previous: None,
            scanner: None,
            errors: Vec::new(),
            panic: false,
            chunk: None,
            compilation: Some(Compilation::new()),
            scope: ScopeTracker::new(),
        }
    }

    pub fn compile(&mut self, source: &str) -> InterpreterResult<Compilation> {
        self.scanner = Some(Scanner::new(source));
        self.chunk = Some(Chunk::new());

        self.advance();
        while !self.match_advance(TokenType::EOF) {
            println!("loop");
            self.declaration();
        }

        self.emit_byte(OpCode::OP_RETURN as u8);

        if self.errors.len() != 0 {
            Err(self.errors.pop().unwrap())
        } else {
            self.compilation.as_mut().unwrap().chunk = Some(self.chunk.take().unwrap());
            let comp = Ok(self.compilation.take().unwrap());

            self.reset();

            comp
        }
    }

    fn emit_byte(&mut self, byte: u8) {
        let line = self.previous.as_ref().unwrap().line;

        self.chunk.as_mut().unwrap().write_byte(byte, line)
    }

    fn emit_bytes(&mut self, byte: u8, byte_operand: u8) {
        self.emit_byte(byte);
        self.emit_byte(byte_operand);
    }

    fn emit_many(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.emit_byte(*b)
        }
    }

    fn emit_const(&mut self, value: Value) {
        let i = self.make_const(value);

        self.emit_bytes(OpCode::OP_CONST as u8, i)
    }

    fn make_const(&mut self, value: Value) -> u8 {
        let i = self.chunk.as_mut().unwrap().add_const(value);

        i as u8
    }

    fn advance(&mut self) {
        self.previous = self.current.take();

        loop {
            self.current = Some(self.scanner.as_mut().unwrap().scan_token());

            if self.current.as_ref().unwrap().token_type != TokenType::ERROR {
                break;
            }

            self.error_at_current("");
        }
    }

    fn error_at_current(&mut self, message: &str) {
        let token = self.current.as_ref().unwrap().clone();
        self.error_at(token, message)
    }

    fn error_at_previous(&mut self, message: &str) {
        let token = self.current.as_ref().unwrap().clone();
        self.error_at(token, message)
    }

    fn error_at(&mut self, token: Token, message: &str) {
        if self.panic {
            return;
        }

        self.panic = true;

        let error = InterpreterError::CompilerError(Error::new(token, message));

        eprintln!("{}", error);

        self.errors.push(error);
    }

    fn consume_if_expected(&mut self, token_type: TokenType, message: &str) {
        if self.current.as_ref().unwrap().token_type == token_type {
            self.advance();
        } else {
            self.error_at_current(message);
        }
    }
}

impl Compiler {
    fn parse_precedence(&mut self, level: Precedence) {
        self.advance();

        let prefix_rule = get_rule(&self.previous.as_ref().unwrap().token_type);
        if prefix_rule.prefix.is_none() {
            self.error_at_current("Expect expression");
            return;
        }

        let is_assignable = level as u8 <= Precedence::ASSIGNMENT as u8;

        self.dispatch(prefix_rule.prefix.as_ref().unwrap(), is_assignable);

        let prec_level = level;
        let prev_level = get_rule(&self.current.as_ref().unwrap().token_type).precedence;
        println!(
            "CALL: {:?} {} CURRENT: {:?} {}",
            prec_level, prec_level as u8, prev_level, prev_level as u8
        );

        while level as u8 <= get_rule(&self.current.as_ref().unwrap().token_type).precedence as u8 {
            self.advance();

            let infix_rule = get_rule(&self.previous.as_ref().unwrap().token_type)
                .infix
                .as_ref()
                .unwrap();

            self.dispatch(infix_rule, is_assignable);
        }

        if is_assignable && self.match_advance(TokenType::EQUAL) {
            self.error_at_current("Invalid assignment target.");
        }
    }

    fn expression(&mut self) {
        self.parse_precedence(Precedence::ASSIGNMENT)
    }

    fn dispatch(&mut self, parse_fn: &ParseFn, is_assignable: bool) {
        match *parse_fn {
            ParseFn::Binary => self.binary(),
            ParseFn::Grouping => self.grouping(),
            ParseFn::Number => self.number(),
            ParseFn::Unary => self.unary(),
            ParseFn::Literal => self.literal(),
            ParseFn::String => self.string(),
            ParseFn::Variable => self.variable(is_assignable),
        }
    }

    fn number(&mut self) {
        let value = self.previous.as_ref().unwrap().lexem.parse::<f64>();
        if let Err(e) = value {
            self.error_at_previous("Invalid number");
        } else {
            self.emit_const(Value::Number(value.unwrap()));
        }
    }

    fn grouping(&mut self) {
        self.expression();

        self.consume_if_expected(TokenType::RIGHT_PAREN, "Expect ')' after expression");
    }

    fn unary(&mut self) {
        let token_type = self.previous.as_ref().unwrap().token_type.clone();

        self.parse_precedence(Precedence::UNARY);

        match token_type {
            TokenType::MINUS => self.emit_byte(OpCode::OP_NEGATE as u8),
            TokenType::BANG => self.emit_byte(OpCode::OP_NOT as u8),
            _ => (),
        }
    }

    fn binary(&mut self) {
        let op = self.previous.as_ref().unwrap().token_type.clone();

        let rule = get_rule(&op);
        let next_prec = rule.get_incremented_prec(1u8);
        if next_prec.is_none() {
            self.error_at_previous("Invalid precedence");
        }

        self.parse_precedence(rule.get_incremented_prec(1u8).unwrap());

        match op {
            TokenType::PLUS => self.emit_byte(OpCode::OP_ADD as u8),
            TokenType::MINUS => self.emit_byte(OpCode::OP_SUB as u8),
            TokenType::STAR => self.emit_byte(OpCode::OP_MUL as u8),
            TokenType::SLASH => self.emit_byte(OpCode::OP_DIV as u8),
            TokenType::BANG_EQUAL => self.emit_bytes(OpCode::OP_EQUAL as u8, OpCode::OP_NOT as u8),
            TokenType::EQUAL_EQUAL => self.emit_byte(OpCode::OP_EQUAL as u8),
            TokenType::GREATER => self.emit_byte(OpCode::OP_GREATER as u8),
            TokenType::GREATER_EQUAL => {
                self.emit_bytes(OpCode::OP_LESS as u8, OpCode::OP_NOT as u8)
            }
            TokenType::LESS => self.emit_byte(OpCode::OP_LESS as u8),
            TokenType::LESS_EQUAL => {
                self.emit_bytes(OpCode::OP_GREATER as u8, OpCode::OP_NOT as u8)
            }

            _ => return,
        }
    }

    fn literal(&mut self) {
        match self.previous.as_ref().unwrap().token_type {
            TokenType::FALSE => self.emit_byte(OpCode::OP_FALSE as u8),
            TokenType::NIL => self.emit_byte(OpCode::OP_NIL as u8),
            TokenType::TRUE => self.emit_byte(OpCode::OP_TRUE as u8),
            _ => return,
        }
    }

    fn string(&mut self) {
        let str_val = self.previous.as_ref().unwrap().lexem.clone();
        let conv_str = String::from(&str_val[1..str_val.len() - 1]);

        self.emit_const(Value::from(conv_str.as_str()))
    }

    fn variable(&mut self, is_assignable: bool) {
        let name = self.extract_str();
        self.named_variable(&name, is_assignable);
    }

    fn declaration(&mut self) {
        if self.match_advance(TokenType::VAR) {
            self.var_declaration();
        } else {
            self.statement();
        }

        if self.panic {
            self.synchronize();
        }
    }

    fn statement(&mut self) {
        if self.match_advance(TokenType::PRINT) {
            self.print_statement();
        } else if self.match_advance(TokenType::LEFT_BRACE) {
            self.scope.begin();
            self.block();
            self.scope.end();
        } else {
            self.expression_statement();
        }
    }

    fn block(&mut self) {
        while !self.check(TokenType::RIGHT_BRACE) && !self.check(TokenType::EOF) {
            self.declaration();
        }

        self.consume_if_expected(TokenType::RIGHT_BRACE, "Expect '}' after block.");
    }

    fn reset(&mut self) {
        self.chunk = None;
        self.compilation = Some(Compilation::new());
        self.previous = None;
        self.current = None;
        self.panic = false;
        self.errors = vec![];
        self.scanner = None;
        self.scope = ScopeTracker::new();
    }

    fn end_scope(&mut self) {
        let pop_count = self.scope.end();
        for _ in 0..pop_count {
           self.emit_byte(OpCode::OP_POP as u8);
        }
    }
}

impl Compiler {
    fn print_statement(&mut self) {
        self.expression();

        self.consume_if_expected(TokenType::SEMICOLON, "Expect ';' after value.");
        self.emit_byte(OpCode::OP_PRINT as u8)
    }

    fn expression_statement(&mut self) {
        self.expression();
        self.consume_if_expected(TokenType::SEMICOLON, "Expect ';' after expression.");
        self.emit_byte(OpCode::OP_POP as u8);
    }

    fn var_declaration(&mut self) {
        let global = self.parse_var("Expect variable name.");

        if self.match_advance(TokenType::EQUAL) {
            self.expression();
        } else {
            self.emit_byte(OpCode::OP_NIL as u8);
        }

        self.consume_if_expected(
            TokenType::SEMICOLON,
            "Expect ';' after variable declaration",
        );
        self.define_var(global);
    }

    fn parse_var(&mut self, error_msg: &str) -> u8 {
        self.consume_if_expected(TokenType::IDENTIFIER, error_msg);
        let var_name = self.extract_str();

        self.declare_var();
        if !self.scope.is_global() {
            return 0;
        }

        self.make_const(Value::from(var_name.as_str()))
    }

    fn declare_var(&mut self) {
        if self.scope.is_global() {
            return;
        }

        let name = self.previous.as_ref().unwrap().clone();

        if self.scope.locate_local(|local_name|  {
            local_name == &name.lexem
        }) != -1 {
            self.error_at_current(&format!("Variable with name '{}' already declared in this scope", name.lexem));
        }

        self.scope.add_local(name);
    }

    fn define_var(&mut self, global: u8) {
        if !self.scope.is_global() {
            self.scope.define_last();
            return;
        }
        self.emit_bytes(OpCode::OP_DEF_GLOBAL as u8, global)
    }

    fn named_variable(&mut self, name: &str, is_assignable: bool) {
        let arg = self.resolve_local(name);
        let get_op = OpCode::OP_GET_LOCAL as u8;
        let set_op = OpCode::OP_SET_LOCAL as u8;
        if arg != -1 {
            ()
        } else {
            let arg = self.make_const(Value::from(name)) as i64;
            let get_op = OpCode::OP_GET_GLOBAL as u8;
            let set_op = OpCode::OP_SET_GLOBAL as u8;
        }

        if is_assignable && self.match_advance(TokenType::EQUAL) {
            self.expression();
            self.emit_bytes(set_op, arg as u8);
        } else {
            self.emit_bytes(get_op, arg as u8);
        }
    }

    fn resolve_local(&mut self, name: &str) -> i64 {
        let i = self.scope.locate_local(|local_name| {
            local_name == name
        });

        if i == -1 {
            self.error_at_current(&format!("Cannot read local variable '{}' in its own initializer", name));
        }

        i
    }
}

impl Compiler {
    fn match_advance(&mut self, token_type: TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.current.as_ref().unwrap().token_type == token_type
    }

    fn check_previous(&self, token_type: TokenType) -> bool {
        self.previous.as_ref().unwrap().token_type == token_type
    }

    fn synchronize(&mut self) {
        self.panic = false;

        while !self.check(TokenType::EOF) {
            if self.check_previous(TokenType::SEMICOLON) {
                return;
            }

            match self.current.as_ref().unwrap().token_type {
                TokenType::RETURN => return,
                _ => (),
            };

            self.advance();
        }
    }

    fn extract_str(&self) -> String {
        let str_val = self.previous.as_ref().unwrap().lexem.clone();
        if str_val.starts_with("\"") {
            String::from(&str_val[1..str_val.len() - 1])
        } else {
            str_val
        }
    }
}
