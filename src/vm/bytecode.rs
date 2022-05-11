use super::value::Value;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
    Return,
}

/// Sequence of bytecode instructions
#[derive(Debug, Clone)]
pub struct Chunk {
    pub code: Vec<Opcode>,
    pub constants: Vec<Value>,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            code: vec![],
            constants: vec![],
        }
    }

    pub fn write_chunk(&mut self, instruction: Opcode) {
        self.code.push(instruction)
    }

    pub fn add_constant(&mut self, value: Value) {
        self.constants.push(value)
    }
}
