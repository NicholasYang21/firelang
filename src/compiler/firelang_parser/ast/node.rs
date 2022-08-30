use crate::compiler::firelang_lexer::lexer::{Token, TokenKind};
use crate::compiler::firelang_lexer::lexer::LiteralKind::*;

use super::token;
struct Literal {
    pub val: token::Literal,
}

impl Literal {
    pub fn new(tok: Token) -> Literal {
        let val: token::Literal;

        match &tok.kind {
            TokenKind::Literal { kind, suffix } => {
                match (kind, suffix.as_str()) {
                    (Int { .. }, suffix) => {
                        match suffix {
                            "" | "i32" => { val = token::Literal::Int(tok); },
                            "u" | "u32" => { val = token::Literal::UInt(tok); },

                            "b" => { val = token::Literal::Byte(tok); },
                            "u8" => { val = token::Literal::UByte(tok); },

                            "i16" => { val = token::Literal::Int16(tok); },
                            "u16" => { val = token::Literal::UInt16(tok); },

                            "i64" => { val = token::Literal::Int64(tok); },
                            "u64" => { val = token::Literal::UInt64(tok); },

                            "i128" => { val = token::Literal::Int128(tok); },
                            "u128" => { val = token::Literal::UInt128(tok); },

                            _ => {
                                val = token::Literal::Unexpected(
                                    "invalid suffix of literal".into()
                                )
                            },
                        }
                    },

                    (Float { dangling }, _) => {
                        if *dangling {
                            val = token::Literal::Unexpected("dangling float number".into())
                        } else {
                            val = unimplemented!()
                        }
                    },

                    (Char { unclose, err }, _) => {

                    },

                    (Str { unclose, err }, _) => {

                    },

                    (RawStr {err }, _) => {

                    }
                }
            }

            _ => (),
        }

        Literal {
            val
        }
    }
}