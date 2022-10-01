use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::compiler::firelang_parser::ast::node::Expression;
use crate::compiler::firelang_parser::ast::token::Literal;

struct LLVMGenerator {
    ctx: Context,
}

impl LLVMGenerator {
    pub fn new() -> Self {
        LLVMGenerator {
            ctx: Context::create()
        }
    }

    fn gen_expr(&mut self, expr: &Expression) {
        match expr {
            Expression::Literal(x) => {
                match x {
                    Literal::Int(val) => {
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
    }
}