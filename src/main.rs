mod io;
mod shell;

use shell::executor;
use shell::parser;

fn main() {
    loop {
        io::prompt::print_prompt();
        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        if bytes_read == 0 || input == "exit" {
            if bytes_read == 0 {
                println!();
            }
            break;
        }

        let commands = parser::parse_input(input);
        executor::execute_command(commands);
    }
    println!("Bye!");
}
