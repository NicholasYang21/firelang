use super::super::super::firelang_errors::gen::gen_error;

use inkwell;
use inkwell::context::Context;
use inkwell::values::AnyValue;

use super::node::*;

pub trait Expr {
    fn codegen(&'static self, ctx: &'static Context) -> Option<Box<dyn AnyValue>>;
}

impl Expr for Literal {
    fn codegen(&'static self, ctx: &'static Context) -> Option<Box<dyn AnyValue>> {

        match &self.val {
            super::token::Literal::Int(tok) => {
                return Some(Box::new(
                    ctx.i32_type().const_int(tok.content.parse::<u64>().unwrap(),
                                             true)));
            }
            _ => {unimplemented!()}
        }
    }
}

impl Expr for Identifier {
    fn codegen(&'static self, ctx: &'static Context) -> Option<Box<dyn AnyValue>> {
        unimplemented!()
    }
}

impl Expr for Error {
    fn codegen(&'static self, _: &'static Context) -> Option<Box<dyn AnyValue>> {
        gen_error(self);
        None
    }
}