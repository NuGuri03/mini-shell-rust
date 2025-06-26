mod io;
mod shell;

use shell::executor;
use shell::parser;
use shell::history::History;

fn main() {
    let mut history = History::new();
    history.load_history("history.txt");

    loop {
        io::prompt::print_prompt();

        let mut input = String::new();
        let bytes_read = std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let input = input.trim();
        history.push(input.to_string());

        if bytes_read == 0 || input == "exit" {
            if bytes_read == 0 {
                println!();
            }
            history.save_history("history.txt");
            break;
        }

        let commands = parser::parse_input(input);
        executor::execute_command(commands);
    }
    println!("Bye!");
}
