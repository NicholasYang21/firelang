use crate::compiler::firelang_parser::ast::token::Literal;

use super::token::BinaryOp;

#[derive(Debug, PartialOrd, PartialEq)]
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

    None
}

#[derive(Debug, PartialOrd, PartialEq)]
pub struct Block {
    pub block: Vec<Statement>,
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Behaviour {
    Copy,
    Move,
    Ref,
}

#[derive(Debug, PartialOrd, PartialEq)]
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
        mutable: bool,
        behaviour: Behaviour,
        value: Expression,
    },

    Return(Expression),

    If {
        cond: Expression,
        block: Block,
        // else
        els: Option<Block>,
    },

    Eof
}
