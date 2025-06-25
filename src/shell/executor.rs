use crate::shell::parser;
use crate::shell::redirect;

fn run_internal_command(command: &parser::Command) -> Result<bool, String> {
    match Some(command.name.as_str()) {
        Some("cd") => {
            if command.args.len() >= 2 {
                return Err("cd: too many arguments".into());
            }
            let target = command
                .args
                .get(1)
                .map(String::as_str)
                .ok_or("cd: missing argument")?;

            std::env::set_current_dir(target)
                .map_err(|e| format!("cd: {e}"))?;

            Ok(true)
        }

        Some("pwd") => {
            let path = std::env::current_dir()
                .map_err(|_| "pwd: failed to get current directory")?;

            println!("{}", path.display());
            Ok(true)
        }

        _ => Ok(false),
    }
}

fn run_external_command(command: &parser::Command) {
    match std::process::Command::new(&command.name)
        .args(&command.args)
        .spawn()
    {
        Ok(mut child) => {
            if let Err(e) = child.wait() {
                eprintln!("bash: failed to wait for process: {}", e);
            }
        }
        Err(_e) => {
            eprintln!("bash: command not found: {}", command.name);
        }
    }
}

pub fn execute_command(commands: Vec<parser::Command>) {
    use nix::libc::dup;

    for cmd in commands {

        let stdin: i32;
        let stdout: i32;

        // backup stdin, stdout
        unsafe {
            stdin = dup(0);
            stdout = dup(1);
        }

        redirect::handle_redirection(&cmd);
        
        match run_internal_command(&cmd) {
            Ok(true) => {},
            Ok(false) => run_external_command(&cmd),
            Err(msg) => eprintln!("{}", msg),
        };

        redirect::close_redirection(stdin, stdout);        
    }
}
