use super::super::super::firelang_errors::gen::gen_error;

use inkwell;
use inkwell::values::AnyValue;

use super::node::*;

pub trait Expr {
    fn codegen<'ctx, T>(&self) -> T where T: AnyValue<'ctx>;
}

impl Expr for Literal {
    fn codegen<'ctx, T>(&self) -> T where T: AnyValue<'ctx> {
        let ctx = inkwell::context::Context::create();

        match self.val {
            super::token::Literal::Int(tok) => {
                return Box::new(
                    ctx.i32_type().const_int(tok.content.parse::<u64>().unwrap(),
                                             true));
            }
            _ => {unimplemented!()}
        }
    }
}

impl Expr for Identifier {
    fn codegen<'ctx, T> (&self) -> T where T: AnyValue<'ctx> {
        unimplemented!()
    }
}

impl Expr for Error {
    fn codegen<'ctx, T>(&self) -> T where T: AnyValue<'ctx> {
        gen_error(self)
    }
}