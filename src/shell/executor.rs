use crate::shell::parser::Command;

fn execute_internal_command(command: &Command) -> bool {
    match command.args.get(0).map(String::as_str) {
        Some("cd") => {
            let target = command.args.get(1).map(String::as_str).unwrap_or_else(|| {
                eprintln!("cd: missing argument");
                return "";
            });

            if !target.is_empty() {
                if let Err(e) = std::env::set_current_dir(target) {
                    eprintln!("cd: {e}");
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
    let command_name = command.args.get(0).map(String::as_str).expect("failed to get command");

    std::process::Command::new(command_name)
        .args(command.args.get(1))
        .spawn()
        .expect("failed to execute process")
        .wait()
        .expect("failed to wait on child");
}

pub fn execute_command(command: Command) {
    if command.argc <= 0 {
        return;
    }

    // pipe

    if execute_internal_command(&command) {
    } else {
        execute_external_command(&command);
    }
}
