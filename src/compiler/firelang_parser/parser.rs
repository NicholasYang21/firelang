use std::ops::{BitXor, BitXorAssign};
use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use crate::compiler::firelang_lexer::lexer::NumBase::Bin;
use crate::compiler::firelang_lexer::lexer::TokenKind::Eof;
use crate::compiler::firelang_parser::ast::node::*;
use crate::compiler::firelang_parser::ast::node_impl::{make_ident, make_lit};
use crate::compiler::firelang_parser::ast::token::BinaryOp;

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(lex: Lexer) -> Parser {
        Parser {
            lex
        }
    }

    fn lookahead(&self) -> Token {
        self.lex.clone().next_token()
    }

    fn next(&mut self) -> Option<Token> {
        let x = self.lex.next_token();
        Some(x)
    }

    fn eat(&mut self) {
        self.lex.next_token();
    }

    fn match_tok(&self, s: &TokenKind) -> bool {
        let mut c = self.lex.clone();

        if c.next_token().kind != *s { return false; }

        true
    }

    fn next_op(&mut self) -> Option<BinaryOp> {
        match self.lookahead().kind {
            TokenKind::Plus => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::AddEq);
                }

                Some(BinaryOp::Add)
            }

            TokenKind::Minus => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::SubEq);
                }

                Some(BinaryOp::Sub)
            }

            TokenKind::Star => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::MulEq);
                }

                Some(BinaryOp::Mul)
            }

            TokenKind::Slash => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::DivEq);
                }

                Some(BinaryOp::Div)
            }

            TokenKind::Percent => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::ModEq);
                }

                Some(BinaryOp::Mod)
            }

            TokenKind::And => {
                self.eat();

                match self.lookahead().kind {
                    TokenKind::And => {
                        self.eat();
                        Some(BinaryOp::LogicalAnd)
                    }

                    TokenKind::Equal => {
                        self.eat();
                        Some(BinaryOp::AndEq)
                    }

                    _ => Some(BinaryOp::And)
                }
            }

            TokenKind::Or => {
                self.eat();

                match self.lookahead().kind {
                    TokenKind::Or => {
                        self.eat();
                        Some(BinaryOp::LogicalOr)
                    }

                    TokenKind::Equal => {
                        self.eat();
                        Some(BinaryOp::OrEq)
                    }

                    _ => Some(BinaryOp::Or)
                }
            }

            TokenKind::Caret => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    Some(BinaryOp::XorEq)
                }

                Some(BinaryOp::Xor)
            }

            TokenKind::Not => {
                self.eat();
                Some(BinaryOp::Not)
            }

            TokenKind::Le => {
                self.eat();
            }

            Eof => None,

            _ => None
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        if self.lookahead().kind == TokenKind::Eof {
            return None;
        }
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> Option<Expression> {
        let x = self.next()?;

        match x.kind {
            TokenKind::Literal { .. } => Some(make_lit(x)),
            TokenKind::Ident { .. } => Some(make_ident(x.content)),
            _ => None
        }
    }
}