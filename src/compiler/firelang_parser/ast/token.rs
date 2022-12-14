use std::fmt::{Display, Formatter};

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
pub enum BinaryOp {
    Add,        // +
    Sub,        // -
    Mul,        // *
    Div,        // /
    Mod,        // %
    Lsh,        // <<
    Rsh,        // >>
    And,        // &
    Or,         // |
    Xor,        // ^
    Not,        // ~
    LogicalNot, // !
    LogicalAnd, // &&
    LogicalOr,  // ||
    Lt,         // <
    Lte,        // <=
    Gt,         // >
    Gte,        // >=
    Eq,         // ==
    Ne,         // !=
    Assign,     // =
    AddEq,      // +=
    SubEq,      // -=
    MulEq,      // *=
    DivEq,      // /=
    ModEq,      // %=
    AndEq,      // &=
    OrEq,       // |=
    XorEq,      // ^=
    LshEq,      // <<=
    RshEq,      // >>=
    Ref,        // ->
    Move,       // <-
    Scope,      // ::
    Is,         // :
}

#[derive(Debug, Eq, PartialEq)]
pub enum KeyWord {
    LET,
    MUT,
    IF,
    ELSE,
    FOR,
    FN,
    RETURN,
}

impl Display for KeyWord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{self:?}").as_str())
    }
}

impl TryFrom<String> for KeyWord {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "let" => Ok(Self::LET),
            "mut" => Ok(Self::MUT),
            "if" => Ok(Self::IF),
            "else" => Ok(Self::ELSE),
            "for" => Ok(Self::FOR),
            "return" => Ok(Self::RETURN),
            "fn" => Ok(Self::FN),
            _ => Err("".into()),
        }
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Literal {
    Byte(i8),
    Int16(i16),
    Int(i32), // int32, default integral type
    Int64(i64),
    Int128(i128),

    UByte(u8),
    UInt16(u16),
    UInt(u32), // uint32, default unsigned integral type
    UInt64(u64),
    UInt128(u128),

    Float(f32), // f32, default floating numeric type
    Float64(f64),

    Char(String),
    Boolean(bool),
    Str(String), // string OR raw string

    Unexpected(String),
}
