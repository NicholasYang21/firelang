use std::fmt::format;
use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use crate::compiler::firelang_parser::ast::node::*;

use super::ast::node::Literal;
use super::ast::codegen::Expr;

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

    fn next(&mut self) -> Token {
        self.lex.next_token()
    }

    fn eat(&mut self) {
        self.lex.next_token();
    }

    fn match_multiple(&self, s: &[TokenKind]) -> bool {
        let mut c = self.lex.clone();

        for i in s {
            if c.next_token().kind != *i { return false; }
        }

        true
    }

    fn parse_literal(&mut self) -> Option<Literal> {
        let x = self.lookahead();

        match x.kind {
            TokenKind::Literal { .. } => { self.eat(); Some(Literal::new(x)) },
            _ => None
        }
    }

    fn parse_identifier(&mut self) -> Option<Identifier> {
        let x = self.lookahead();

        match x.kind {
            TokenKind::Ident { .. } => { self.eat(); Some(Identifier::new(x)) },
            _ => None
        }
    }

    pub fn parse_primary(&mut self) -> Primary {
        if let Some(x) = self.parse_literal() {
            return Primary {
                prim: Box::new(x)
            };
        }

        if let Some(x) = self.parse_identifier() {
            return Primary {
                prim: Box::new(x)
            }
        }

        let x = self.lookahead();

        Primary {
            prim: Box::new(Error {
                msg: format!("expect <literal> or <identifier>, but there is '{}'", self.lookahead().content),
                short: "unexpected expression".into(),
                line: self.lex.source.as_str().lines().nth(self.lex.line - 1).unwrap().to_string(),
                col: x.column,
                ln: x.line,
                len: x.content.len()
            })
        }
    }
}