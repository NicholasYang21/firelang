use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use crate::compiler::firelang_parser::ast::node::*;

use super::ast::node::Literal;
use super::ast::codegen::Expr;

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

    fn create_error(&self, msg: String, short: &str, tok: Token) -> Error {
        Error {
            msg,
            short: short.into(),
            line: self.lex.src.as_str().lines().nth(self.lex.line - 1).unwrap().to_string(),
            col: tok.column,
            ln: tok.line,
            len: tok.content.len()
        }
    }

    pub fn parse(&mut self) -> Option<Primary> {
        if self.lookahead().kind == TokenKind::Eof {
            return None;
        }
        Some(self.parse_primary())
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

        self.next();

        Primary {
            prim: Box::new(self.create_error(
                format!("expected <identifier>, <literal> or <expr>, but there is '{}'", x.content),
                "unexpected token",
                    x
            ))
        }
    }
}