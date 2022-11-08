use crate::compiler::firelang_parser::ast::token::Literal;

use super::token::BinaryOp;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),

    Binary {
        lhs: Box<Expression>,
        op: BinaryOp,
        rhs: Box<Expression>,
    },

    Ident(String),

    FuncCall {
        ident: String,
        args: Vec<Expression>,
    },
}

pub struct Block {
    block: Vec<Statement>,
}

pub enum Statement {
    Block(Block),

    FuncDecl {
        ident: String,
        params: Vec<Expression>,
        body: Block,
    },

    VariableDecl {
        ident: String,
        ty: String,
        value: Expression,
    },

    Return(Expression),

    If {
        cond: Expression,
        block: Block,
        // else
        els: Option<Block>,
    },
}
