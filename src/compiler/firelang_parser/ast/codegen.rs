use super::node::*;

pub trait Expr {
    fn codegen(&self) {}
}

impl Expr for Literal {

}

impl Expr for Identifier {

}

impl Expr for Error {
    fn codegen(&self) {
        println!("\x1b[1;31mError: \x1b[0m{}", self.msg);

        let len = self.ln.to_string().len() + 1;
        let mut white: String = "".into();

        for _ in 0..len {
            white.push(' ');
        }

        println!("\x1b[38;2;0;127;0m{white}|\n{} | \x1b[0m{}", self.ln, self.line);

        let spaces = white;
        white = " ".into();

        for _ in 0..self.col - 1 {
            white.push(' ');
        }

        print!("{}\x1b[38;2;0;127;0m|{white}\x1b[38;2;255;0;0m^", spaces);

        for _ in 0..self.len - 1 {
            print!("_")
        }

        println!(" \x1b[1;38;2;255;0;0m{}\x1b[0m", self.short);
    }
}