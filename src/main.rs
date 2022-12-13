use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::parser::Parser;
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    std::fs::File::open("./test/var_decl.test")
        .unwrap()
        .read_to_string(&mut buffer)
        .expect("ERROR");

    let lexer = Lexer::new(buffer.as_str());
    let mut parser = Parser::new(lexer);
    let mut counter = 0;

    while counter <= 12 {
        let expr = parser.parse();

        println!("{:#?}", expr);

        counter += 1;
    }

}
