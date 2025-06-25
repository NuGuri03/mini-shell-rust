use crate::parser::Command;

use std::fs::File;
use std::os::unix::io::AsRawFd;
use nix::libc::{dup2, close};

pub fn handle_redirection(cmd: &Command) {
    
    if let Some(ref input) = cmd.stdin {
        let input_file = File::open(input).expect("failed to open file");
        unsafe {
            dup2(input_file.as_raw_fd(), 0);
        }
    }

    if let Some(ref output) = cmd.stdout {
        let output_file = File::create(output).expect("failed to create file");
        unsafe {
            dup2(output_file.as_raw_fd(), 1);
        }
    }
}

pub fn close_redirection(stdin: i32, stdout: i32) {
    unsafe {
        dup2(stdin, 0);
        dup2(stdout, 1);
        close(stdin);
        close(stdout);
    }
}