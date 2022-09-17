use crate::compiler::firelang_lexer::lexer::{Lexer, Token, TokenKind};
use crate::compiler::firelang_parser::ast::node::*;
use crate::compiler::firelang_parser::ast::node_impl::make_lit;

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

    fn match_tok(&self, s: &TokenKind) -> bool {
        let mut c = self.lex.clone();

        if c.next_token().kind != *s { return false; }

        true
    }

    fn parse_literal(&mut self) -> Option<Expression> {
        let x = self.lookahead();

        match x.kind {
            TokenKind::Literal { .. } => { self.eat(); Some(make_lit(x)) },
            _ => None
        }
    }

    fn parse_identifier(&mut self) -> Option<Expression> {
        let x = self.lookahead();

        match x.kind {
            TokenKind::Ident { .. } => { self.eat(); Some(make_lit(x)) },
            _ => None
        }
    }

    pub fn parse(&mut self) -> Option<Expression> {
        if self.lookahead().kind == TokenKind::Eof {
            return None;
        }
        Some(self.parse_expr())
    }

    pub fn parse_expr(&mut self) -> Option<Expression> {

    }
}