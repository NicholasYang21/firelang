use std::fs;
use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;

fn main() {
    let buffer = "*";
    let lexer = Lexer::new(buffer);
    let mut parser = Parser::new(lexer);

    let x = parser.parse_primary();

    x.prim.codegen();
}
