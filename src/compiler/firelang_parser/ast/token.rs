use std::fmt::{Display, Formatter};
use crate::compiler::firelang_lexer::lexer::Token;

#[derive(Debug)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Lsh, // <<
    Rsh, // >>
    And, // &
    Or, // |
    Xor, // ^
    Not, // ~
    LogicalNot, // !
    LogicalAnd, // &&
    LogicalOr, // ||
    Lt, // <
    Lte, // <=
    Gt, // >
    Gte, // >=
    Eq, // ==
    Ne, // !=
    Assign, // =
    AddEq, // +=
    SubEq, // -=
    MulEq, // *=
    DivEq, // /=
    ModEq, // %=
    AndEq, // &=
    OrEq, // |=
    XorEq, // ^=
    LshEq, // <<=
    RshEq, // >>=
}

#[derive(Debug, Eq, PartialEq)]
pub enum KeyWord {
    IMM,
    MUT,
    IF,
    ELSE,
    FOR
}

impl Display for KeyWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self).as_str())
    }
}

impl TryFrom<String> for KeyWord {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "imm" => Ok(Self::IMM),
            "mut" => Ok(Self::MUT),
            "if" => Ok(Self::IF),
            "else" => Ok(Self::ELSE),
            "for" => Ok(Self::FOR),
            _ => Err("".into())
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Byte(Token),
    Int16(Token),
    Int(Token), // int32, default integral type
    Int64(Token),
    Int128(Token),

    UByte(Token),
    UInt16(Token),
    UInt(Token), // uint32, default unsigned integral type
    UInt64(Token),
    UInt128(Token),

    Float(Token), // f32, default floating numeric type
    Float64(Token),

    Char(Token),
    Boolean(Token),
    Str(Token), // string OR raw string

    Unexpected(String),
}