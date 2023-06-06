use std::io;
use std::io::{stdout, Write};
// use std::str::Chars;
// use std::process::Command;

pub mod lexer;
pub mod parser;
use crate::lexer::tokenize;
use crate::parser::Parser;

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
            if !token_list.is_empty() {
                let mut parser = Parser::new(token_list);
                match parser.parse() {
                    Ok(cmd) => { println!("{:?}", cmd); },
                    Err(msg) => { println!("error: {}", msg); },
                }
            }
            hist += 1;
        }
    }
}
