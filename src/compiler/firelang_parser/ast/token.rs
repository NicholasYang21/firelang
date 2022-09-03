use crate::compiler::firelang_lexer::lexer::Token;

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
    LogicalAnd, // &&
    LogicalOr, // ||
    Lt, // <
    Lte, // <=
    Gt, // >
    Gte, // >=
    Eq, // ==
    Ne, // !=
}

pub enum AssignOp {
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