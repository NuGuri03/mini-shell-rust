use std::io::{self, Write};

pub fn print_prompt() {
    print!("mysh> ");
    io::stdout().flush().expect("Failed to stdout");
}