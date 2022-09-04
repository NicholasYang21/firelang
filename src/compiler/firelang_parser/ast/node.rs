use crate::compiler::firelang_lexer::lexer::Token;

use super::token::BinaryOp;

use super::codegen::Expr;
use super::token;

pub struct Literal {
    pub val: token::Literal
}

pub struct Identifier {
    pub ident: Token
}

pub struct Primary {
    pub prim: Box<dyn Expr>,
}

pub struct Error {
    pub msg: String,
    pub short: String,
    pub line: String,
    pub col: usize,
    pub ln: usize,
    pub len: usize,
}

pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Primary,
    pub right: Primary
}