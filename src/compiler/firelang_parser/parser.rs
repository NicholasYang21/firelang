use crate::compiler::firelang_lexer::lexer::{Lexer, Token};
pub struct Parser<'a> {
    lexer : Lexer<'a>,
    prev : Option<Token>
}

impl Lexer<'_> {
    //TODO: Add funtcions
}