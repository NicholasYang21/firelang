use super::super::super::firelang_errors::gen::gen_error;

use inkwell;

use super::node;
use super::node::*;

pub trait Expr {
    fn codegen(&self) -> Value;
}

impl Expr for Literal {
    fn codegen(&self) {
        let ctx = inkwell::context::Context::create();
        match self.val {
            super::token::Literal::Int(tok) => {
                return ctx.i32_type().const_int(tok.content.parse::<u64>().unwrap(), false);
            }
            _ => {unimplemented!()}
        }
    }
}

impl Expr for Identifier {

}

impl Expr for Error {
    fn codegen(&self) {
        gen_error(self);
    }
}