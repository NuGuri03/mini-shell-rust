use std::io::{self, Write};

pub fn print_prompt() {
    // TODO: add cuurent_path
    // TODO: 
    print!("mysh> ");
    io::stdout().flush().expect("Failed to stdout");
}
