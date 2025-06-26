use std::io::{self, Read, Write};
use termios::*;

use crate::shell::history::History;
use crate::io::prompt;

pub fn enable_raw_mode() -> Termios {
    let stdin = 0;
    let mut termios = Termios::from_fd(stdin).expect("failed to get termios");
    let original = termios.clone();

    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(stdin, TCSANOW, &termios).unwrap();

    original
}

pub fn disable_raw_mode(original: &Termios) {
    tcsetattr(0, TCSANOW, original).unwrap();
}

pub fn read_input(history: &mut History) -> String {
    let mut command = String::new();

    loop {
        let mut buffer = [0; 1];
        io::stdin().read_exact(&mut buffer).expect("failed to read byte");
        let byte = buffer[0];
    
        match byte {
            // EOF
            0x04 => {
                println!();
                return String::from("__EOF__");
            }

            // Enter('\n)
            b'\n' => {
                println!();
                if !command.trim().is_empty() {
                    history.push(command.clone());
                }
                return command;
            }
    
            // backspace
            0x7f => {
                if !command.is_empty() {
                    command.pop();
                    print!("\x08 \x08");
                    io::stdout().flush().unwrap();
                }
            }
    
            // escape sequence
            0x1b => {
                let mut seq = [0; 2];
                io::stdin().read_exact(&mut seq).unwrap();

                if seq == [91, 65] {
                    // up arrow
                    if let Some(i) = history.index {
                        if i > 0 {
                            history.index = Some(i - 1);
                        }
                    } 
                    if let Some(i) = history.index {
                        command = history.entries[i].clone();
                        print!("\r\x1b[2K");
                        prompt::print_prompt(command.as_str());
                    }
                } else if seq == [91, 66] {
                    // down arrow
                    if let Some(i) = history.index {
                        if i + 1 < history.entries.len() {
                            history.index = Some(i + 1);
                            command = history.entries[history.index.unwrap()].clone();
                        }
                    }

                    print!("\r\x1b[2K");
                    prompt::print_prompt(command.as_str());
                }
            }
    
            _ => {
                command.push(byte as char);
                print!("{}", byte as char);
                io::stdout().flush().unwrap();
            }
        }
    }
}