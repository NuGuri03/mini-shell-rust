#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
}

#[derive(Debug)]
enum ParseState {
    Normal,
    InSingleQuote,
    InDoubleQuote,
    Escape,
}

fn tokenize_input(input: &str) -> Vec<String> {
    use ParseState::*;
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut state = Normal;

    for ch in input.chars() {
        match state {
            Normal => match ch {
                '"' => state = InDoubleQuote,
                '\'' => state = InSingleQuote,
                '\\' => state = Escape,
                ' ' | '\t' => {
                    if !current.is_empty() {
                        tokens.push(current.clone());
                        current.clear();
                    }
                }
                _ => current.push(ch),
            },

            InSingleQuote => match ch {
                '\'' => state = Normal,
                '\\' => state = Escape,
                _ => current.push(ch),
            },

            InDoubleQuote => match ch {
                '"' => state = Normal,
                '\\' => state = Escape,
                _ => current.push(ch),
            },

            Escape => {
                current.push(ch);
                state = Normal;
            }
        }
    }

    if !current.is_empty() {
        tokens.push(current);
    }

    tokens
}

fn parse_tokens(tokens: Vec<String>) -> Command {
    let mut args = Vec::new();
    let mut stdin = None;
    let mut stdout = None;

    let mut i = 0;
    while i < tokens.len() {
        let token = tokens[i].clone();
        if token == ">" {
            i += 1;
            if i < tokens.len() {
                stdout = Some(tokens[i].clone());
            } else {
                eprintln!(">: failed to redirect")
            }
        } else if token == "<" {
            i += 1;
            if i < tokens.len() {
                stdin = Some(tokens[i].clone());
            } else {
                eprintln!("<: failed to redirect")
            }
        } else {
            args.push(token);
        }
        i += 1;
    }
    let name = args.remove(0);

    Command {
        name,
        args,
        stdin,
        stdout,
    }
}

pub fn parse_input(input: &str) -> Vec<Command> {
    let tokens = tokenize_input(input);

    let mut cmds = Vec::new();
    let mut cmd_tokens = Vec::new();
    for token in &tokens {
        if token == "|" {
            cmds.push(parse_tokens(cmd_tokens.clone()));
            cmd_tokens.clear();
        } else {
            cmd_tokens.push(token.clone());
        }
    }

    if !cmd_tokens.is_empty() {
        cmds.push(parse_tokens(cmd_tokens));
    }

    cmds
}
