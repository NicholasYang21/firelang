use crate::compiler::firelang_lexer::lexer::Token;
use super::token;

struct PrimaryNode {
    pub literal: Option<Token>,
    pub expr: Option<ExprNode>,
}

impl PrimaryNode {
    pub fn Gen() {
        let a = inkwell::context::Context::create();
    }
}

struct MulExpr {
    pub left: ExprNode,
    pub op: Token,
    pub right: ExprNode,
}

struct ExprNode {

}