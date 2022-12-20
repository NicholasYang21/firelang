use std::collections::HashMap;
enum OpType {
    Add,
    Mul,
}

pub struct ByteCode {
    opcode: OpType
}

impl ByteCode {
    pub fn cast_to_code(&self) {
        
    }
}

pub struct VM {
    // Context
    pub symbols: HashMap<String, (u64 /* address */, u64 /* length */)>,
    pub functions: Vec<u64>,
    // Base
    pub registers: [u64; 16],
    pub memory: Vec<u64>,
    pub pc: usize,
}
