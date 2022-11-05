/*use std::io::stdout;
use std::io::Write;
use winapi_util::console::{Color, Console, Intense};

macro_rules! set_color {
    ($console: expr, $color: expr, $other: stmt) => {
        if let Ok(..) = $console {
            stdout().flush().unwrap();
            $console.as_mut().unwrap().reset().unwrap();
            $console.as_mut().unwrap().fg(Intense::Yes, $color).unwrap();
        } else {
            $other
        }
    }
}

macro_rules! reset {
    ($console: expr) => {
        set_color!($console, Color::White, print!("\x1b[0m"))
    }
}

fn print_line(ln: usize, line: &String) {
    let mut console = Console::stdout();

    set_color!(console, Color::Green, print!("\x1b[0;32m"));

    for _ in 0..=ln {
        print!(" ")
    }

    println!("|\n{ln} | {line}");

    reset!(console);
}

fn print_error(msg: String) {
    let mut console = Console::stdout();

    set_color!(console, Color::Red, print!("\x1b[1;31m"));
    print!("Error: ");

    reset!(console);
    println!("{msg}");
}

fn print_tail(short: String, ln: usize, col: usize, len: usize) {
    let mut console = Console::stdout();

    set_color!(console, Color::Green, print!("\x1b[0;32m"));

    for _ in 0..=ln {
        print!(" ");
    }

    print!("|");

    for _ in 0..=col - 1 {
        print!(" ");
    }

    set_color!(console, Color::Cyan, print!("\x1b[0;36m"));
    print!("^");

    for _ in 0..len - 1 {
        print!("_");
    }

    set_color!(console, Color::Red, print!("\x1b[1;31m"));
    println!(" {short}\n");
    reset!(console)
}*/
