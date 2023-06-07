use std::process;
use std::os::fd::RawFd;
use std::os::unix::process::CommandExt;
use nix::errno::Errno;
use nix::Result;
use nix::fcntl::{OFlag, open};
use nix::unistd::{ForkResult::*, Pid, fork, pipe, close, dup2};
use nix::sys::wait::{WaitStatus, wait};
use nix::sys::stat::Mode;
// use crate::syscall::{Result, WaitStatus, Error, wait};
// use crate::fork::Fork::fork;

use crate::parser::{Command, WriteMode};
use Command::*;
use WriteMode::*;

pub fn invoke(cmd: &Command) -> Result<WaitStatus> {
    match cmd {
        // Sequence { lhs, rhs }
        Pipe(vec) => {
            let n_proc = vec.len();
            let n_pipe = vec.len() - 1;
            let mut fd: Vec<(RawFd, RawFd)> = Vec::new();
            let mut pid_last = Pid::from_raw(0); // unused value
            for _i in 0..n_pipe {
                fd.push(pipe()?);
            }
            for i in 0..n_proc {
                match unsafe { fork()? } {
                    Child => {
                        if i > 0 { dup2(fd[i-1].0, 0)?; }
                        if i < n_pipe { dup2(fd[i].1, 1)?; }
                        for i in 0..n_pipe {
                            close(fd[i].0)?;
                            close(fd[i].1)?;
                        }
                        redirect_exec(&vec[i]);
                    },
                    Parent { child: pid } => {
                        pid_last = pid;
                    },
                }
            }

            for i in 0..n_pipe {
                close(fd[i].0)?;
                close(fd[i].1)?;
            }

            let mut status_last = WaitStatus::Exited(pid_last, 0); // unused value
            for _i in 0..n_proc {
                let wait_status = wait()?;
                if let WaitStatus::Exited(pid, _) = wait_status {
                    if pid == pid_last {
                        status_last = wait_status;
                    }
                }
            }

            Ok(status_last)
        },
        Redirect { .. } => {
            match unsafe { fork()? } {
                Child => redirect_exec(cmd),
                Parent { child: _pid } => wait(),
            }
        },
        _ => { panic!(""); },
    }
}

fn redirect_exec(cmd: &Command) -> Result<WaitStatus> {
    match cmd {
        Redirect { cmd: cmd2, input, output } => {
            if let Some(name) = input {
                let fd_in = open(&**name, OFlag::O_RDONLY, Mode::S_IRWXU)?;
                dup2(fd_in, 0)?;
                close(fd_in)?;
            }
            if let Some((write_mode, name)) = output {
                let fd_out = match write_mode {
                    Output => open(&**name, OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_TRUNC, Mode::S_IRWXU)?,
                    Append => open(&**name, OFlag::O_WRONLY | OFlag::O_CREAT | OFlag::O_APPEND, Mode::S_IRWXU)?,
                };
                dup2(fd_out, 1)?;
                close(fd_out)?;
            }
            match &**cmd2 {
                Simple { name, args } => {
                    process::Command::new(name).args(args).exec();
                },
                _ => {},
            }
        },
        _ => {},
    }
    Err(Errno::EPERM) // tekitou
}