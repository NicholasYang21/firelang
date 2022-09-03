use super::node::*;

pub trait Expr {
    fn codegen(&self) {}
}

impl Expr for Literal {

}

impl Expr for Identifier {

}