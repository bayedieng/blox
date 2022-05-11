pub mod bytecode;
mod value;

use bytecode::{Chunk, Opcode};
use value::Value;

type InterpreterResult = Result<(), Error>;

macro_rules! binop {
    ($self:ident, $op:tt) => {{
        let b = $self.stack.pop().unwrap();
        let a = $self.stack.pop().unwrap();
        let value = a $op b;
        $self.stack.push(value);
    }};
}

#[derive(Debug)]
pub enum Error {
    CompileError,
    RuntimeError,
}
#[derive(Debug, Clone)]
pub struct VM {
    chunk: Chunk,
    pc: usize,
    stack: Vec<Value>,
}

impl VM {
    pub fn new() -> VM {
        VM {
            chunk: Chunk::new(),
            pc: 0,
            stack: vec![],
        }
    }

    fn run(&mut self) -> InterpreterResult {
        loop {
            let instruction = self.chunk.code[self.pc];
            match instruction {
                Opcode::Constant(value) => {
                    println!("{value}");
                    self.stack.push(value)
                }
                Opcode::Return => return Ok(()),
                Opcode::Negate => {
                    let value = -self.stack.pop().unwrap();
                    self.stack.push(value)
                }
                Opcode::Add => binop!(self, +),
                Opcode::Subtract => binop!(self, -),
                Opcode::Multiply => binop!(self, *),
                Opcode::Divide => binop!(self, /),
            }
            self.pc += 1;
        }
    }

    pub fn interpret(&mut self, chunk: Chunk) -> InterpreterResult {
        self.chunk = chunk;
        self.run()
    }
}
