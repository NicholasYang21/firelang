use inkwell::context::Context;
/*use inkwell::types::AnyType;
use inkwell::values::AnyValue;

use crate::compiler::firelang_parser::ast::node::Expression;
use crate::compiler::firelang_parser::ast::token::Literal;*/

struct _LLVMGenerator {
    ctx: Context,
}

impl _LLVMGenerator {
    pub fn _new() -> Self {
        _LLVMGenerator {
            ctx: Context::create(),
        }
    }

    /*fn gen_expr(&mut self, expr: &Expression) -> Box<dyn AnyValue> {
        match expr {
            Expression::Literal(x) => {
                match x {
                    Literal::Int(val) => {
                        Box::from(self.ctx.i32_type().const_int(*val as u64, true))
                    }

                    Literal::UInt(val) => {

                    }

                    _ => { unimplemented!() }
                }
            }

            Expression::Ident(x) => {

            }

            Expression::Binary { lhs, op, rhs } => {

            }
        }
    }*/
}
