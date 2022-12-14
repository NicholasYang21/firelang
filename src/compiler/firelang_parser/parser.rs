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
        (BinaryOp::Scope, 15),
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

    fn match_keyword(&self, s: &KeyWord) -> Result<(), String> {
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

                if self.lookahead().kind == TokenKind::Ge {
                    self.eat();
                    return Some(BinaryOp::Ref);
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

                    TokenKind::Minus => {
                        self.eat();
                        Some(BinaryOp::Move)
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

            TokenKind::Equal => {
                self.eat();
                Some(BinaryOp::Assign)
            }

            TokenKind::Colon => {
                self.eat();

                if self.lookahead().kind == TokenKind::Colon {
                    self.eat();
                    return Some(BinaryOp::Scope);
                }

                Some(BinaryOp::Is)
            }

            TokenKind::Eof => None,

            _ => None,
        }
    }

    pub fn parse(&mut self) -> Result<Statement, String> {
        self.parse_stmt()
    }

    pub fn has_content(&self) -> bool {
        self.lookahead().kind != TokenKind::Eof
    }

    fn parse_literal(&mut self) -> Result<Expression, String> {
        if let Some(x) = self.next() {
            if let TokenKind::Literal { .. } = x.kind {
                return Ok(make_lit(x));
            }
        }

        Err("Error: Expected <literal> but there is EOF.".into())
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

    fn parse_ident(&mut self) -> Result<Token, String> {
        let x = self.next().unwrap();

        if x.kind != TokenKind::Ident {
            return Err("Expected an <identifier>.".into());
        }

        Ok(x)
    }

    fn parse_ident_or_call(&mut self) -> Result<Expression, String> {
        let ident: Expression;
        let mut args: Vec<Expression> = Vec::new();

        let x = self.parse_ident()?;

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
                    return Err("Error: Expected a ',' after the argument".into());
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
            _ => Err("Error: Unexpected token: Expected <literal>, <identifier> or '('.".into()),
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
                return Err("Error: Expected <literal>, <identifier> or '(' after operator.".into());
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

    pub fn parse_stmt(&mut self) -> Result<Statement, String> {
        let mut result: Result<Statement, String>;

        if self.match_keyword(&KeyWord::FN).is_ok() {
            self.eat();
            result = self.parse_func_decl();
        } else if self.match_keyword(&KeyWord::LET).is_ok() {
            self.eat();
            result = self.parse_var_decl();
            if self.match_tok(&TokenKind::Semicolon).is_err() {
                result = Err("There must be a ';' after a <statement>.".into());
            }
        } else if self.match_tok(&TokenKind::LeftBrace).is_ok() {
            self.eat();
            result = self.parse_block();
        } else if self.match_keyword(&KeyWord::RETURN).is_ok() {
            self.eat();
            result = self.parse_return();
        } else {
            if self.lookahead().kind == TokenKind::Eof {
                return Ok(Statement::Eof);
            }
            result = Err("Error: Expected <statement>.".into());
        }

        result.as_ref()?;

        self.eat();

        result
    }

    pub fn parse_func_decl(&mut self) -> Result<Statement, String> {
        let name = self.parse_ident()?;
        let mut params: Vec<(String, Behaviour, String)> = Vec::new();

        if self.lookahead().kind != TokenKind::LeftParen {
            return Err("Expected a '(' after the function name.".into());
        }

        self.eat(); // eat '('.

        while self.lookahead().kind != TokenKind::RightParen {
            if self.lookahead().kind == TokenKind::Comma {
                self.eat(); // eat ',' between the parameters.
            }

            let param = {
                let param_name = self.parse_ident()?;
                let behaviour = self.next_tok_is_op();

                if behaviour.is_none() {
                    return Err("Expected '=', '->' or '<-' after the parameter name".into());
                }

                let bhv: Behaviour = match behaviour.unwrap() {
                    BinaryOp::Assign => Behaviour::Copy,
                    BinaryOp::Ref => Behaviour::Ref,
                    BinaryOp::Move => Behaviour::Move,
                    _ => return Err("Expected '=', '->' or '<-' after the parameter name".into()),
                };

                let ty = self.parse_ident()?;

                (param_name.content, bhv, ty.content)
            };

            params.push(param);
        }

        self.eat(); // eat ')'.

        if self.lookahead().kind != TokenKind::LeftBrace {
            return Err("Expected a block as function body after function signature.".into());
        }

        self.eat(); // eat '{'.

        if let Ok(Statement::Block(body)) = self.parse_block() {
            Ok(Statement::FuncDecl {
                ident: name.content,
                params,
                body,
            })
        } else {
            Err("Expected a block as function body after function signature.".into())
        }
    }

    // "let" ("mut") <ident>(":" <type: ident>) ({ "=" | "<-" | "->" } <expr>)
    pub fn parse_var_decl(&mut self) -> Result<Statement, String> {
        let mut mutable = false;
        let ident: String;
        let mut ty: String = "".into();

        if self.match_keyword(&KeyWord::MUT).is_ok() {
            self.eat();
            mutable = true;
        }

        if let Ok(Expression::Ident(x)) = self.parse_ident_or_call() {
            ident = x;
        } else {
            return Err("Expected <identifier> after keyword 'let'.".into());
        }

        if self.match_tok(&TokenKind::Colon).is_ok() {
            self.eat();
            if let Ok(Expression::Ident(x)) = self.parse_ident_or_call() {
                ty = x;
            } else {
                return Err("Expected <type-name> after ':' in variable declaring.".into());
            }
        }

        if let Some(o) = self.next_tok_is_op() {
            let behaviour = match o {
                BinaryOp::Assign => Behaviour::Copy,
                BinaryOp::Ref => Behaviour::Ref,
                BinaryOp::Move => Behaviour::Move,
                _ => {
                    return Err("The assignment operator must be '=', '->' or '<-' ".into());
                }
            };

            let rhs = self.parse_expr();

            if rhs.is_err() {
                return Err("Error: Value of the variable must be a expression.".into());
            }

            let value = rhs.unwrap();

            Ok(Statement::VariableDecl {
                ident,
                ty,
                mutable,
                behaviour,
                value,
            })
        } else {
            Err("Must initialize the variable when declare it.".into())
        }
    }

    pub fn parse_return(&mut self) -> Result<Statement, String> {
        let expr = self.parse_expr();

        if expr.is_err() {
            return Err("Error: Expected <expr> after keyword 'return'.".into());
        }

        Ok(Statement::Return(expr.unwrap()))
    }

    pub fn parse_block(&mut self) -> Result<Statement, String> {
        let mut block: Block = Block { block: Vec::new() };

        while self.lookahead().kind != TokenKind::RightBrace {
            let x = self.parse_stmt()?;

            block.block.push(x);
        }
        self.eat();

        Ok(Statement::Block(block))
    }
}
