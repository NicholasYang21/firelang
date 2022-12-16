use firelang::compiler::firelang_lexer::lexer::*;
use firelang::compiler::firelang_parser::ast::node::Statement;
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

    loop {
        let expr = parser.parse();
        if let Ok(expr) = expr {
            if expr != Statement::Eof {
                println!("{:#?}", expr);
            } else {
                break;
            }
        } else {
            println!("{}", expr.unwrap_err());
            break;
        }
    }
}
