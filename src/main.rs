
use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;

fn main() {
    let buffer = "55555";
    let lexer = Lexer::new(buffer);
    let mut parser = Parser::new(lexer);

    while parser.clone().parse().is_some() {
        let x = parser.parse().unwrap();
        x.prim.codegen();
    }
}
