use std::fs;
use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_lexer::lexer::TokenKind::*;

fn main() {
    let buf : String = fs::read_to_string("./test/lexer.fire").unwrap();
    let mut lexer = Lexer::new(buf.as_str());

    loop {
        let t = lexer.advance_token();
        println!("{:#?}", t);

        if t.kind == Eof {
            break;
        }
    }
}
