use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use std::collections::HashMap;

use crate::compiler::firelang_parser::ast::node::Expression::Literal;
use crate::compiler::firelang_parser::ast::node::*;
use crate::compiler::firelang_parser::ast::node_impl::{make_ident, make_lit};
use crate::compiler::firelang_parser::ast::token;
use crate::compiler::firelang_parser::ast::token::{BinaryOp, KeyWord};

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,
}

impl Parser<'_> {
    pub fn new(lex: Lexer) -> Parser {
        Parser { lex }
    }

    fn lookahead(&self) -> Token {
        self.clone().next().unwrap()
    }

    fn next(&mut self) -> Option<Token> {
        let x = self.lex.next_token();

        if x.kind == TokenKind::Space {
            return self.next();
        }

        if x.kind == TokenKind::Eof {
            return None;
        }

        Some(x)
    }

    fn eat(&mut self) {
        self.next().unwrap();
    }

    fn match_tok(&mut self, s: &TokenKind) -> Result<(), String> {
        let k = self.lookahead().kind;

        if k != *s {
            return Err(format!(
                "At line {:?}, col {:?}: Expected {:?}, found {:?}",
                self.lex.line, self.lex.column, s, k
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

                    _ => Some(BinaryOp::And),
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

                    _ => Some(BinaryOp::Or),
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

                    _ => Some(BinaryOp::Lt),
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

                    _ => Some(BinaryOp::Gt),
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

            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<Expression, String> {
        self.parse_expr()
    }

    fn parse_literal(&mut self) -> Result<Expression, String> {
        if let Some(x) = self.next() {
            if let TokenKind::Literal { .. } = x.kind {
                return Ok(make_lit(x));
            }
        }
        Err("Expect <literal> but there is EOF.".into())
    }

    fn parse_paren(&mut self) -> Result<Expression, String> {
        self.eat();
        let expr = self.parse_expr()?;

        if self.lookahead().kind == TokenKind::RightParen {
            return Ok(expr);
        }

        Err("Unexpected unclosed '('.".into())
    }

    fn parse_ident(&mut self) -> Result<Expression, String> {
        unimplemented!()
    }

    pub fn parse_primary(&mut self) -> Result<Expression, String> {
        unimplemented!()
    }

    pub fn parse_expr(&mut self) -> Result<Expression, String> {
        let x = self.next();

        if x != None {
        } else {
            return Err("Error: unexpected EOF.".into());
        }

        let x = x.unwrap();

        let expr = match x.kind {
            TokenKind::LeftParen => {
                let x = self.parse_expr()?;
                self.match_tok(&TokenKind::RightParen)?;

                x
            }

            TokenKind::Literal { .. } => make_lit(x),
            TokenKind::Ident => {
                if let Ok(x) = KeyWord::try_from(x.content.clone()) {
                    return Err(format!(
                        "At line {:?}, col {:?}: Unexpected keyword <{:?}>.",
                        self.lex.line, self.lex.column, x
                    ));
                }

                match x.content.as_str() {
                    "true" => Literal(token::Literal::Boolean(true)),
                    "false" => Literal(token::Literal::Boolean(false)),
                    _ => make_ident(x.content),
                }
            }

            _ => {
                return Err(format!(
                    "At line {:?}, col {:?}: Expected <expression>, found {:?}.",
                    self.lex.line, self.lex.column, x.kind
                ))
            }
        };

        if let Some(op) = self.next_tok_is_op() {
            self.parse_binary_expr(expr, op)
        } else {
            Ok(expr)
        }
    }

    fn parse_binary_expr(&mut self, expr: Expression, op: BinaryOp) -> Result<Expression, String> {
        let temp_rhs = self.parse_expr()?;

        let precedence: HashMap<BinaryOp, i32> = vec![
            (BinaryOp::OrEq, 0),
            (BinaryOp::AndEq, 0),
            (BinaryOp::XorEq, 0),
            (BinaryOp::LshEq, 1),
            (BinaryOp::RshEq, 1),
            (BinaryOp::AddEq, 2),
            (BinaryOp::SubEq, 2),
            (BinaryOp::MulEq, 3),
            (BinaryOp::DivEq, 3),
            (BinaryOp::ModEq, 3),
            (BinaryOp::Assign, 4),
            (BinaryOp::LogicalOr, 5),
            (BinaryOp::LogicalAnd, 6),
            (BinaryOp::LogicalNot, 6),
            (BinaryOp::Lt, 7),
            (BinaryOp::Lte, 7),
            (BinaryOp::Gt, 7),
            (BinaryOp::Gte, 7),
            (BinaryOp::Eq, 8),
            (BinaryOp::Ne, 8),
            (BinaryOp::Or, 9),
            (BinaryOp::Xor, 10),
            (BinaryOp::And, 11),
            (BinaryOp::Not, 11),
            (BinaryOp::Lsh, 12),
            (BinaryOp::Rsh, 12),
            (BinaryOp::Add, 13),
            (BinaryOp::Sub, 13),
            (BinaryOp::Mul, 14),
            (BinaryOp::Div, 14),
            (BinaryOp::Mod, 14),
        ]
        .into_iter()
        .collect();

        if let Expression::Binary {
            lhs: _,
            op: op2,
            rhs: _,
        } = &temp_rhs
        {
            return if precedence[op2] < precedence[&op] {
                Ok(Expression::Binary {
                    lhs: Box::from(expr),
                    op,
                    rhs: Box::from(temp_rhs),
                })
            } else {
                Ok(Expression::Binary {
                    lhs: Box::from(temp_rhs),
                    op,
                    rhs: Box::from(expr),
                })
            };
        }

        Ok(Expression::Binary {
            lhs: Box::from(expr),
            op,
            rhs: Box::from(temp_rhs),
        })
    }
}
