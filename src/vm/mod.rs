mod bytecode;
mod value;

use bytecode::{Chunk, Opcode};

type InterpreterResult = Result<(), Error>;

#[derive(Debug)]
pub enum Error {
    CompileError,
    RuntimeError,
}
#[derive(Debug, Clone)]
pub struct VM {
    chunk: Chunk,
    pc: usize,
}

impl VM {
    pub fn new(chunk: Chunk) -> VM {
        VM {
            chunk: chunk,
            pc: 0,
        }
    }

    fn run(&mut self) -> InterpreterResult {
        loop {
            let instruction = self.chunk.code[self.pc];
            match instruction {
                Opcode::Constant(value) => println!("{value}"),
                Opcode::Return => return Ok(()),
            }
            self.pc += 1;
        }
    }
}
