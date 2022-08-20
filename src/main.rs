use std::fs;
use firelang::compiler::firelang_lexer::lexer::*;

fn main() {
    let buffer = fs::read_to_string("test/lexer_test.test").unwrap();
    let mut lexer = Lexer::new(buffer.as_str());

    loop {
        let token = lexer.next_token();

        println!("{:#?}", token);

        if token.kind == TokenKind::Eof {
            break;
        }
    }
}
