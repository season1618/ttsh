use std::io;
use std::io::{stdout, Write};
// use std::str::Chars;
// use std::process::Command;

pub mod lexer;
use crate::lexer::tokenize;

fn main() {
    let mut hist = 0;
    let mut line: String;
    loop {
        print!("ttsh[{}]> ", hist);
        stdout().flush().unwrap();

        line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            println!("{}", line);
            let token_list = tokenize(&line);
            println!("{:?}", token_list);
            hist += 1;
        }
    }
}
