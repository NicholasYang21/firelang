use std::io::Read;
use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;

fn main() {
    let mut buffer = String::new();
    std::fs::File::open("./test/binary_parsing.test")
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("ERROR");

    let lexer = Lexer::new(buffer.as_str());
    let mut parser = Parser::new(lexer);
    let expr = parser.parse();

    println!("{:#?}", expr);
}
