use std::process;
use std::os::unix::process::CommandExt;
use nix::errno::Errno;
use nix::Result;
use nix::unistd::{ForkResult::*, fork};
use nix::sys::wait::{WaitStatus, wait};
// use crate::syscall::{Result, WaitStatus, Error, wait};
// use crate::fork::Fork::fork;

#[no_std]

use crate::parser::{Command, OutputFile};
use Command::*;
use OutputFile::*;

pub fn invoke(cmd: &Command) -> Result<WaitStatus> {
    match cmd {
        // Sequence { lhs, rhs }
        Pipe(vec) => {
            invoke(&vec[0])
        },
        Redirect { cmd: cmd2, input, output } => {
            match &**cmd2 {
                Simple { name, args } => {
                    match unsafe { fork() } {
                        Ok(Child) => { process::Command::new(name).args(args).exec(); panic!(""); },
                        Ok(Parent { child: pid_ }) => wait(),
                        Err(errno) => Err(errno),
                    }
                },
                _ => { panic!(""); },
            }
        },
        _ => { panic!(""); },
    }
}