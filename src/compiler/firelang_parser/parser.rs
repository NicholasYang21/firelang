use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};

use crate::compiler::firelang_parser::ast::node::*;
use crate::compiler::firelang_parser::ast::node::Expression::Literal;
use crate::compiler::firelang_parser::ast::node_impl::{make_ident, make_lit};
use crate::compiler::firelang_parser::ast::token;
use crate::compiler::firelang_parser::ast::token::{BinaryOp, KeyWord};

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
        self.clone().next().unwrap()
    }

    fn next(&mut self) -> Option<Token> {
        let x = self.lex.next_token();

        if x.kind == TokenKind::Space { return self.next(); }

        Some(x)
    }

    fn eat(&mut self) {
        self.next().unwrap();
    }

    fn match_tok(&mut self, s: &TokenKind) -> Result<(), String> {
        let k = self.lookahead().kind;

        if k != *s {
            return Err(format!(
            "At line {:?}, col {:?}: Expected {:?}, found {:?}", self.lex.line, self.lex.column, s, k
        ));
        }

        Ok(())
    }

    fn _match_keyword(&self, s: &KeyWord) -> Result<(), String> {
        let k = self.lookahead();

        if let Ok(x) = KeyWord::try_from(k.content.clone()) {
            if x != *s {
                return Err(format!(
                    "At line {:?}, col {:?}: Expected keyword <{}>, found keyword <{}>",
                    self.lex.line, self.lex.column, s, x
                ));
            }
        } else {
            return Err(format!(
                "At line {:?}, col {:?}: Expected keyword <{:?}>, found {:?}",
                self.lex.line, self.lex.column, s, k
            ));
        }

        Ok(())
    }

    fn next_tok_is_op(&mut self) -> Option<BinaryOp> {
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
                    return Some(BinaryOp::XorEq);
                }

                Some(BinaryOp::Xor)
            }

            TokenKind::Not => {
                self.eat();
                Some(BinaryOp::Not)
            }

            TokenKind::Le => {
                self.eat();

                match self.lookahead().kind {
                    TokenKind::Le => {
                        self.eat();

                        if self.lookahead().kind == TokenKind::Equal {
                            self.eat();
                            return Some(BinaryOp::LshEq);
                        }

                        Some(BinaryOp::Lsh)
                    }

                    TokenKind::Equal => {
                        self.eat();
                        Some(BinaryOp::Lte)
                    }

                    _ => Some(BinaryOp::Lt)
                }
            }

            TokenKind::Ge => {
                self.eat();

                match self.lookahead().kind {
                    TokenKind::Ge => {
                        self.eat();

                        if self.lookahead().kind == TokenKind::Equal {
                            self.eat();
                            return Some(BinaryOp::RshEq);
                        }

                        Some(BinaryOp::Rsh)
                    }

                    TokenKind::Equal => {
                        self.eat();
                        Some(BinaryOp::Gte)
                    }

                    _ => Some(BinaryOp::Gt)
                }
            }

            TokenKind::Exclamation => {
                self.eat();

                if self.lookahead().kind == TokenKind::Equal {
                    self.eat();
                    return Some(BinaryOp::Ne);
                }

                Some(BinaryOp::LogicalNot)
            }

            TokenKind::Eof => None,

            _ => None
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expr()
    }

    pub fn parse_expr(&mut self) -> Result<Expression, String> {
        let x = self.next().unwrap();

        let expr = match x.kind {
            TokenKind::LeftParen => {
                let x = self.parse_expr()?;
                self.match_tok(&TokenKind::RightParen)?;

                x
            },

            TokenKind::Literal { .. } => make_lit(x),
            TokenKind::Ident => {
                if let Ok(x) = KeyWord::try_from(x.content.clone()) {
                    return Err(format!(
                        "At line {:?}, col {:?}: Unexpected keyword <{:?}>.",
                        self.lex.line, self.lex.column, x)
                    )
                }

                match x.content.as_str() {
                    "true" => Literal(token::Literal::Boolean(true)),
                    "false" => Literal(token::Literal::Boolean(false)),
                    _ => make_ident(x.content)
                }
            }

            _ => return Err(format!(
                "At line {:?}, col {:?}: Expected <expression>, found {:?}.",
                self.lex.line, self.lex.column, x.kind)
            ),
        };

        if let Some(op) = self.next_tok_is_op() {
            self.parse_binary_expr(expr, op)
        } else {
            Ok(expr)
        }
    }

    fn parse_binary_expr(&mut self, expr: Expression, op: BinaryOp) -> Result<Expression, String> {
        Ok(Expression::Binary {
            lhs: Box::from(expr),
            op,
            rhs: Box::from(self.parse_expr()?),
        })
    }
}
