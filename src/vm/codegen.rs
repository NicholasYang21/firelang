use crate::compiler::firelang_parser::ast::node::*;


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