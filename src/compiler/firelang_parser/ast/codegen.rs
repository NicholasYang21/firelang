use super::node::*;
use super::super::super::firelang_errors::gen::gen_error;

pub trait Expr {
    fn codegen(&self) {}
}

impl Expr for Literal {

}

impl Expr for Identifier {

}

impl Expr for Error {
    fn codegen(&self) {
        gen_error(self);
    }
}