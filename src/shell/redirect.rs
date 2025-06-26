use crate::parser::Command;

use nix::libc::{close, dup2};
use std::fs::File;
use std::os::unix::io::AsRawFd;

pub fn handle_redirection(cmd: &Command) {
    if let Some(ref input) = cmd.stdin {
        let input_file = File::open(input).expect("failed to open file");
        unsafe {
            if dup2(input_file.as_raw_fd(), 0) == -1 {
                let err = nix::errno::Errno::last();
                panic!("dup2 failed to redirect stdin: {err}");
            }
        }
    }

    if let Some(ref output) = cmd.stdout {
        let output_file = File::create(output).expect("failed to create file");
        unsafe {
            if dup2(output_file.as_raw_fd(), 0) == -1 {
                let err = nix::errno::Errno::last();
                panic!("dup2 failed to redirect stdin: {err}");
            }
        }
    }
}

pub fn close_redirection(stdin: i32, stdout: i32) {
    unsafe {
        if dup2(stdin, 0) == -1 {
            let err = nix::errno::Errno::last();
            panic!("dup2 failed to restore stdin: {err}");
        }

        if dup2(stdout, 1) == -1 {
            let err = nix::errno::Errno::last();
            panic!("dup2 failed to restore stdout: {err}");
        }

        close(stdin);
        close(stdout);
    }
}
