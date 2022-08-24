use std::fs;
use firelang::compiler::firelang_lexer::lexer::*;
use std::time::*;

fn main() {
    let buffer = fs::read_to_string("test/lexer_test.fire").unwrap();
    let mut lexer = Lexer::new(buffer.as_str());

    let bef_now = SystemTime::now();
    loop {
        let token = lexer.next_token();

        if token.kind == TokenKind::Eof {
            break;
        }
    }

    println!("time use {:?}", SystemTime::now().duration_since(bef_now).expect("反方向的钟").as_nanos());
}
