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
    let args = Vec::new();

    Command::new(args)
}
