use std::io::{self, Write};

use colored::Colorize;

pub fn print_prompt() {
    let username = whoami::username();
    let host = whoami::devicename();
    let current_path = std::env::current_dir()
        .expect("failed to get current dir")
        .display()
        .to_string();

    let user_host = format!("{}@{}", username, host).bold().red();
    let path = current_path.blue();

    let prompt = format!("{}:{}$ ", user_host, path);

    print!("{}", prompt);
    io::stdout().flush().expect("failed to stdout");
}
