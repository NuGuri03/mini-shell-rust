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

fn execute_single_command(command: &parser::Command) {
    use nix::libc::dup;

    let stdin: i32;
    let stdout: i32;

    // backup stdin, stdout
    unsafe {
        stdin = dup(0);
        stdout = dup(1);
    }

    redirect::handle_redirection(&command);
    
    match run_internal_command(&command) {
        Ok(true) => {},
        Ok(false) => run_external_command(&command),
        Err(msg) => eprintln!("{}", msg),
    };

    redirect::close_redirection(stdin, stdout);    
}

pub fn execute_command(commands: Vec<parser::Command>) {
    if commands.len() == 1 {
        let command = &commands[0];
        execute_single_command(command);
        return;    
    }

    use std::process;

    let mut previous_stdout = None;
    let mut children = Vec::new();

    for (i, cmd) in commands.iter().enumerate() {
        let mut process = process::Command::new(&cmd.name);
        process.args(&cmd.args);

        if let Some(stdout) = previous_stdout.take() {
            process.stdin(stdout);
        }

        if i < commands.len() - 1 {
            process.stdout(process::Stdio::piped());
        } else {
            if let Some(ref output_file) = cmd.stdout {
                let file = std::fs::File::create(output_file).expect("failed to create file");
                process.stdout(process::Stdio::from(file));
            }
        }

        if i == 0 {
            if let Some(ref input_file) = cmd.stdin {
                let file = std::fs::File::open(input_file).expect("failed to open file");
                process.stdin(process::Stdio::from(file));
            }
        }

        let mut child = process.spawn().expect("failed to spawn child");
        previous_stdout = child.stdout.take().map(process::Stdio::from);
        children.push(child);
    }

    for mut child in children {
        child.wait().expect("failed to wait child");
    }
}