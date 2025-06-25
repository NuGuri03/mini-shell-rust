use crate::shell::parser::Command;

fn execute_internal_command(command: &Command) -> bool {
    match Some(command.name.as_str()) {
        Some("cd") => {
            if command.args.len() >= 2 {
                eprintln!("cd: too many arguments");
                return true;
            }
            let target = command.args.get(1).map(String::as_str).unwrap_or_else(|| {
                eprintln!("cd: missing argument");
                return "";
            });

            if !target.is_empty() {
                if let Err(e) = std::env::set_current_dir(target) {
                    eprintln!("cd: {e}");
                    return true;
                }
            }
            true
        }

        Some("pwd") => {
            if let Ok(path) = std::env::current_dir() {
                println!("{}", path.display());
            } else {
                eprintln!("pwd: failed to get current directory");
            }
            true
        }
        _ => false,
    }
}

fn execute_external_command(command: &Command) {
    std::process::Command::new(&command.name)
        .args(&command.args)
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on child");
}

pub fn execute_command(commands: Vec<Command>) {
    for cmd in commands {
        println!("{:?}", cmd);
        if !execute_internal_command(&cmd) {
            execute_external_command(&cmd);
        }
    }
}
