use std::io;
use std::io::{stdout, Write};
// use std::str::Chars;
// use std::process::Command;
use std::vec::IntoIter;
use std::iter::Peekable;

use Token::*;

const SEP: [char; 7] = ['|', '&', ';', '<', '>', '(', ')'];

#[derive(Debug)]
enum Token {
    DoubleVerticalLine,
    DoubleAmpersand,
    SemiColon,
    VerticalLine,
    Lt,
    Gt,
    Gg,
    OpenParen,
    CloseParen,
    Str(String),
}

struct Lexer {
    cur: Peekable<IntoIter<char>>,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer { cur: "".to_string().chars().collect::<Vec<char>>().into_iter().peekable() }
    }

    fn tokenize(&mut self, line: &String) -> Vec<Token> {
        self.cur = line.chars().collect::<Vec<char>>().into_iter().peekable();
        let mut token_list: Vec<Token>  = Vec::new();
        while let Some(token) = self.next_token() {
            token_list.push(token);
        }
        token_list
    }

    fn next_token(&mut self) -> Option<Token> {
        while let Some(c) = self.cur.peek() {
            if c.is_whitespace() {
                self.cur.next();
                continue;
            }
            break;
        }
        match self.cur.next() {
            Some('|') => {
                match self.cur.peek() {
                    Some('|') => {
                        self.cur.next();
                        Some(DoubleVerticalLine)
                    },
                    _ => Some(VerticalLine),
                }
            },
            Some('&') => {
                match self.cur.peek() {
                    Some('&') => {
                        self.cur.next();
                        Some(DoubleAmpersand)
                    },
                    _ => None,
                }
            },
            Some(';') => Some(SemiColon),
            Some('<') => Some(Lt),
            Some('>') => {
                match self.cur.peek() {
                    Some('>') => {
                        self.cur.next();
                        Some(Gg)
                    },
                    _ => Some(Gt),
                }
            },
            Some('(') => Some(OpenParen),
            Some(')') => Some(CloseParen),
            Some(c) => {
                let mut s = String::new();
                s.push(c);
                while let Some(&d) = self.cur.peek() {
                    if SEP.to_vec().iter().find(|&&x| x == d) == None && !d.is_whitespace() {
                        self.cur.next();
                        s.push(d);
                    } else {
                        break;
                    }
                }
                Some(Token::Str(s))
            },
            None => None,
        }
    }
}

fn main() {
    let mut hist = 0;
    let mut line: String;
    let mut lexer = Lexer::new();
    loop {
        print!("ttsh[{}]> ", hist);
        stdout().flush().unwrap();

        line = String::new();
        if let Ok(_) = io::stdin().read_line(&mut line) {
            println!("{}", line);
            let token_list = lexer.tokenize(&line);
            println!("{:?}", token_list);
            hist += 1;
        }
    }
}
