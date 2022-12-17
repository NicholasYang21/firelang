use crate::compiler::firelang_parser::ast::node::*;

enum OpType {
    Add,
    Sub,
    Mul,
}

pub struct ByteCode {
    opcode: OpType
}

pub trait Generator {
    fn gen(&mut self) -> ByteCode;
}

impl Generator for Expression {
    fn gen(&mut self) -> ByteCode {
        match &self {
            Expression::Binary { lhs, op, rhs} => {

            }

            _ => (),
        }

        unimplemented!()
    }
}