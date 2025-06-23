#[derive(Debug)]
pub struct Command {
    args: Vec<String>,
    argc: usize,
}

impl Command {
    pub fn new(args: Vec<String>) -> Self {
        let argc = args.len();
        Self { args, argc }
    }
}

pub fn parse_input(input: &str) -> Command {
    let parsed_input = input
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();

    Command::new(parsed_input)
}
