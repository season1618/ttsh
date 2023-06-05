use std::vec::IntoIter;
use std::iter::Peekable;

use Token::*;

const SEP: [char; 7] = ['|', '&', ';', '<', '>', '(', ')'];

#[derive(Debug, PartialEq)]
pub enum Token {
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

pub fn tokenize(line: &String) -> Vec<Token> {
    let mut token_list: Vec<Token>  = Vec::new();
    let mut cur = line.chars().collect::<Vec<char>>().into_iter().peekable();
    while let Some(token) = next_token(&mut cur) {
        token_list.push(token);
    }
    token_list
}

fn next_token(cur: &mut Peekable<IntoIter<char>>) -> Option<Token> {
    while let Some(c) = cur.peek() {
        if c.is_whitespace() {
            cur.next();
            continue;
        }
        break;
    }
    match cur.next() {
        Some('|') => {
            match cur.peek() {
                Some('|') => {
                    cur.next();
                    Some(DoubleVerticalLine)
                },
                _ => Some(VerticalLine),
            }
        },
        Some('&') => {
            match cur.peek() {
                Some('&') => {
                    cur.next();
                    Some(DoubleAmpersand)
                },
                _ => None,
            }
        },
        Some(';') => Some(SemiColon),
        Some('<') => Some(Lt),
        Some('>') => {
            match cur.peek() {
                Some('>') => {
                    cur.next();
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
            while let Some(&d) = cur.peek() {
                if SEP.to_vec().iter().find(|&&x| x == d) == None && !d.is_whitespace() {
                    cur.next();
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