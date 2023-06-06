use std::rc::Rc;
use std::vec::IntoIter;
use std::iter::Peekable;

use crate::lexer::Token;
use Token::*;
use Command::*;
use OutputFile::*;

#[derive(Debug)]
pub enum Command {
    Sequence { lhs: Rc<Command>, rhs: Rc<Command> },
    BranchAnd { lhs: Rc<Command>, rhs: Rc<Command> },
    BranchOr { lhs: Rc<Command>, rhs: Rc<Command> },
    Pipe(Vec<Command>),
    Redirect { cmd: Rc<Command>, input: Option<String>, output: Option<(OutputFile, String)> },
    Subshell(Rc<Command>),
    Simple { name: String, args: Vec<String> },
}

#[derive(Debug)]
pub enum OutputFile {
    Output,
    Append,
}

pub struct Parser {
    cur: Peekable<IntoIter<Token>>,
}

impl Parser {
    pub fn new(token_list: Vec<Token>) -> Self {
        Parser { cur: token_list.into_iter().peekable() }
    }

    pub fn parse(&mut self) -> Result<Command, String> {
        self.parse_redirect()
        // self.parse_sequence()
    }

    // fn parse_sequence(&mut self) -> Command {
    //     let lhs = self.parse_branch();
    //     if self.expect(SemiColon) {
    //         let rhs = self.parse_sequence();
    //         return Sequence { lhs: Rc::new(lhs), rhs: Rc::new(rhs) };
    //     } else {
    //         return lhs;
    //     }
    // }

    // fn parse_branch(&mut self) -> Command {
    //     let mut lhs = self.parse_pipe();
    //     loop {
    //         if self.expect(DoubleAmpersand) {
    //             let rhs = self.parse_pipe();
    //             lhs = BranchAnd { lhs: Rc::new(lhs), rhs: Rc::new(rhs) };
    //             continue;
    //         }
    //         if self.expect(DoubleVerticalLine) {
    //             let rhs = self.parse_pipe();
    //             lhs = BranchOr { lhs: Rc::new(lhs), rhs: Rc::new(rhs) };
    //             continue;
    //         }
    //         break;
    //     }
    //     Pipe(lhs)
    // }

    // fn parse_pipe(&mut self) -> Command {
    //     let mut vec = vec![self.parse_redirect()];
    //     while self.expect(VerticalLine) {
    //         let cmd = self.parse_redirect();
    //         vec.push(cmd);
    //     }
    //     return vec;
    // }

    fn parse_redirect(&mut self) -> Result<Command, String> {
        let cmd = self.parse_primary()?;
        let mut input: Option<String> = None;
        let mut output: Option<(OutputFile, String)> = None;
        loop {
            if self.expect(Lt) {
                input = Some(self.read_str()?);
                continue;
            }
            if self.expect(Gt) {
                output = Some((Output, self.read_str()?));
                continue;
            }
            if self.expect(Gg) {
                output = Some((Append, self.read_str()?));
                continue;
            }
            break;
        }
        Ok(Redirect { cmd: Rc::new(cmd), input, output })
    }

    fn parse_primary(&mut self) -> Result<Command, String> {
        // if self.expect(OpenParen) {
        //     let cmd = self.parse_sequence();
        //     self.consume(CloseParen);
        // } else {
            let name = self.read_str()?;
            let mut args: Vec<String> = Vec::new();
            while let Some(arg) = self.expect_str() {
                args.push(arg);
            }
            return Ok(Simple { name, args });
        // }
    }

    fn expect(&mut self, token: Token) -> bool {
        if self.cur.peek() == Some(&token) {
            self.cur.next();
            return true;
        } else {
            return false;
        }
    }

    fn expect_str(&mut self) -> Option<String> {
        let result = match self.cur.peek() {
            Some(Str(name)) => Some(name.clone()),
            _ => return None,
        };
        self.cur.next();
        result
    }

    fn consume(&mut self, token: Token) -> Result<(), String> {
        if self.cur.next() == Some(token) {
            return Ok(());
        } else {
            return Err(String::from("unexpected token"));
        }
    }

    fn read_str(&mut self) -> Result<String, String> {
        if let Some(Str(name)) = self.cur.next() {
            return Ok(name);
        }
        Err(String::from("string is expected"))
    }
}