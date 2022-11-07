use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::compiler::firelang_parser::ast::node::*;
use crate::compiler::firelang_parser::ast::node_impl::{make_ident, make_lit};
use crate::compiler::firelang_parser::ast::token::{BinaryOp, KeyWord};

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,
}

static PRECEDENCE: Lazy<HashMap<BinaryOp, i32>> = Lazy::new(|| {
    vec![
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
    .collect()
});

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

        Some(x)
    }

    fn eat(&mut self) {
        self.next().unwrap();
    }

    fn _match_tok(&mut self, s: &TokenKind) -> Result<(), String> {
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

        Err("Error: Expect <literal> but there is EOF.".into())
    }

    fn parse_paren(&mut self) -> Result<Expression, String> {
        self.eat();
        let expr = self.parse_expr()?;

        if self.lookahead().kind == TokenKind::RightParen {
            self.eat();
            return Ok(expr);
        }

        Err("Error: Unclosed '('.".into())
    }

    fn parse_ident_or_call(&mut self) -> Result<Expression, String> {
        let ident: Expression;
        let mut args: Vec<Expression> = Vec::new();

        let x = self.next().unwrap();
        ident = make_ident(x.content.clone());

        if self.lookahead().kind != TokenKind::LeftParen {
            return Ok(ident);
        }

        self.eat();
        if self.lookahead().kind != TokenKind::RightParen {
            loop {
                if let Ok(arg) = self.parse_expr() {
                    args.push(arg);
                } else {
                    return Err("Error: arguments for a function must be an expression.".into());
                }

                if self.lookahead().kind == TokenKind::RightParen {
                    break;
                }

                if self.lookahead().kind != TokenKind::Comma {
                    return Err("Error: expected a ',' after the argument".into());
                }
                self.eat();
            }
        }

        self.eat();

        Ok(Expression::FuncCall {
            ident: x.content,
            args,
        })
    }

    pub fn parse_primary(&mut self) -> Result<Expression, String> {
        match self.lookahead().kind {
            TokenKind::Literal { .. } => self.parse_literal(),
            TokenKind::Ident { .. } => self.parse_ident_or_call(),
            TokenKind::LeftParen => self.parse_paren(),
            _ => Err("Error: unexpected token: expect <literal>, <identifier> or '('.".into()),
        }
    }

    pub fn parse_expr(&mut self) -> Result<Expression, String> {
        let lhs = self.parse_primary();
        lhs.as_ref()?;

        self.parse_binary_expr(0, lhs.unwrap())
    }

    fn parse_binary_expr(&mut self, in_p: i32, mut lhs: Expression) -> Result<Expression, String> {
        loop {
            let tok = self.next_tok_is_op();
            let p = {
                if let Some(..) = tok {
                    PRECEDENCE[&tok.clone().unwrap()]
                } else {
                    -1
                }
            };

            if p < in_p {
                return Ok(lhs);
            }

            let mut rhs = self.parse_primary();
            if rhs.is_err() {
                return Err(format!(
                    "Error: expect <literal>, <identifier> or '(' after operator. {:#?}",
                    rhs
                ));
            }

            let p2 = {
                let temp = self.clone().next_tok_is_op();

                if let Some(..) = temp {
                    PRECEDENCE[&temp.unwrap()]
                } else {
                    -1
                }
            };

            if p < p2 {
                rhs = self.parse_binary_expr(in_p + 1, rhs.unwrap());
                rhs.as_ref()?;
            }

            lhs = Expression::Binary {
                lhs: Box::new(lhs),
                op: tok.unwrap(),
                rhs: Box::new(rhs.unwrap()),
            };
        }
    }
}
