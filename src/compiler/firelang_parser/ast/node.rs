use crate::compiler::firelang_parser::ast::token::Literal;

use super::token::BinaryOp;

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),

    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>
    },

    Ident(String),
}