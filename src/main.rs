use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;

fn main() {
    let buffer = "1 + 2 * 3";
    let lexer = Lexer::new(buffer);
    let mut parser = Parser::new(lexer);
    let expr = parser.parse();

    println!("{:#?}", expr);
}
