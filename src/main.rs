use inkwell::context::Context;
use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;

fn main() {
    let ctx = Context::create();
    let buffer = "55555";
    let lexer = Lexer::new(buffer);
    let mut parser = Parser::new(lexer);

}
