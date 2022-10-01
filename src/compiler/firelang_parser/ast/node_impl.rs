use crate::compiler::firelang_lexer::lexer::{Token, TokenKind};
use crate::compiler::firelang_lexer::lexer::LiteralKind::*;
use crate::compiler::firelang_lexer::lexer::RawStrError::*;

use crate::compiler::firelang_lexer::unescape::UnescapeError;
use crate::compiler::firelang_lexer::unescape::UnescapeError::*;

use super::node::*;
use super::token;


pub fn make_lit(tok: Token) -> Expression {
    fn check_escape_err(err: &UnescapeError) -> token::Literal {
        match err {
            OnlyOneSlashError =>
                token::Literal::Unexpected(
                    r"only one '\' in character literal".into()),

            IllegalEscape =>
                token::Literal::Unexpected(
                    "there are some illegal characters in the escape sequence".into()),

            EmptyUnicode =>
                token::Literal::Unexpected(
                    "unexpected empty unicode escape sequence".into()),

            UnclosedUnicode =>
                token::Literal::Unexpected(
                    "there is not a '}' to close the escape sequence".into()),

            IllegalUnicode =>
                token::Literal::Unexpected(
                    "there is an illegal unicode escape sequence in the character literal".into()),

            TooLongUnicode =>
                token::Literal::Unexpected(
                    "too long value in the unicode escape sequence".into()),

            ValueOutOfUnicode =>
                token::Literal::Unexpected(
                    "value out of range: unicode value cannot be more than 10FFFF".into()),

            LoneSurrogate =>
                token::Literal::Unexpected(
                    "there is a lone surrogate codepoint in the unicode character".into()),

            InvalidCharInUnicode =>
                token::Literal::Unexpected(
                    "there is an invalid character in the unicode escape sequence".into()),

            TooShortEscape =>
                token::Literal::Unexpected(
                    "too short escape sequence in ascii escape character".into()),

            InvalidCharInHex =>
                token::Literal::Unexpected(
                    "there is an invalid character in the ascii escape sequence".into()),

            ValueOutOfHex =>
                token::Literal::Unexpected(
                    "value out of range: ascii escape value cannot be more than 0x7F".into()),
        }
    }

    let val: token::Literal = match &tok.kind {
        TokenKind::Literal { kind, suffix } => {
            match (kind, suffix.as_str()) {
                (Int { .. }, suffix) => {
                    match suffix {
                        "" | "i32" => {
                            token::Literal::Int(
                                tok.content.replace("i32", "").parse::<i32>().unwrap()
                            )
                        },

                        "u32" => {
                            token::Literal::UInt(
                                tok.content.replace("u32", "").parse::<u32>().unwrap()
                            )
                        },

                        "b" => {
                            token::Literal::Byte(
                                tok.content.replace("b", "").parse::<i8>().unwrap()
                            )
                        },

                        "u8" => {
                            token::Literal::UByte(
                                tok.content.replace("u8", "").parse::<u8>().unwrap()
                            )
                        },

                        "i16" => {
                            token::Literal::Int16(
                                tok.content.replace("i16", "").parse::<i16>().unwrap()
                            )
                        },

                        "u16" => {
                            token::Literal::UInt16(
                                tok.content.replace("u16", "").parse::<u16>().unwrap()
                            )
                        },

                        "i64" => {
                            token::Literal::Int64(
                                tok.content.replace("i64", "").parse::<i64>().unwrap()
                            )
                        },

                        "u64" => {
                            token::Literal::UInt64(
                                tok.content.replace("u64", "").parse::<u64>().unwrap()
                            )
                        },

                        "i128" => {
                            token::Literal::Int128(
                                tok.content.replace("i128", "").parse::<i128>().unwrap()
                            )
                        },

                        "u128" => {
                            token::Literal::UInt128(
                                tok.content.replace("u128", "").parse::<u128>().unwrap()
                            )
                        },

                        _ => {
                            token::Literal::Unexpected(
                                "invalid suffix of literal".into()
                            )
                        },
                    }
                },

                (Float { dangling }, suffix) => {
                    if *dangling {
                        token::Literal::Unexpected("dangling float number".into())
                    } else {
                        match suffix {
                            "f32" | "" => {
                                token::Literal::Float(
                                    tok.content.replace("f32", "").parse::<f32>().unwrap()
                                )
                            },

                            "f64" => {
                                token::Literal::Float64(
                                    tok.content.replace("f64", "").parse::<f64>().unwrap()
                                )
                            },

                            _ => token::Literal::Unexpected(
                                "invalid suffix of literal".into()
                            ),
                        }
                    }
                },

                (Char { unclose, err }, _) => {
                    if *unclose {
                        token::Literal::Unexpected("unclose character literal".into())
                    } else {
                        match err {
                            Some(err) => check_escape_err(err),

                            None => token::Literal::Char(tok.content)
                        }
                    }
                },

                (Str { unclose, err }, _) => {
                    if *unclose {
                        token::Literal::Unexpected("unclose character literal".into())
                    } else {
                        match err {
                            Some(err) => check_escape_err(err),

                            None => token::Literal::Str(tok.content)
                        }
                    }
                },

                (RawStr {err }, _) => {
                    match err {
                        Some(err) => {
                            match err {
                                UncloseString => token::Literal::Unexpected(
                                    "the string literal is unclosed".into()),

                                UncloseParen => token::Literal::Unexpected(
                                    "missed a parentheses in the raw string literal".into()
                                ),
                            }
                        },
                        None => token::Literal::Str(tok.content),
                    }
                }
            }
        }

        _ => token::Literal::Unexpected("Unexpected literal".into()),
    };

    Expression::Literal(val)
}

pub fn make_ident(s: String) -> Expression {
    Expression::Ident(s)
}

