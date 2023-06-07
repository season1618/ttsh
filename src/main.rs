use std::io;
use std::io::{stdout, Write};

pub mod lexer;
pub mod parser;
pub mod invoke;
use crate::lexer::tokenize;
use crate::parser::Parser;
use crate::invoke::invoke;

fn main() {
    let mut hist = 0;
    let mut line: String;
    loop {
        print!("ttsh[{}]> ", hist);
        stdout().flush().unwrap();

        line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            let token_list = tokenize(&line);
            if !token_list.is_empty() {
                let mut parser = Parser::new(token_list);
                match parser.parse() {
                    Ok(cmd) => { invoke(&cmd); },
                    Err(msg) => println!("error: {}", msg),
                }
            }
            hist += 1;
        }
    }
}
