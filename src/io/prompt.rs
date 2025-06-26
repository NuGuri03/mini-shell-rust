use std::io::{self, Write};

use colored::Colorize;

pub fn print_prompt() {
    let current_path = std::env::current_dir()
        .expect("failed to get current dir")
        .to_str()
        .expect("failed to parse &str")
        .to_string();

    print!("{}:{}$ ", "host".red(), current_path.blue());
    io::stdout().flush().expect("failed to stdout");
}
