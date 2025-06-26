mod io;
mod shell;

use shell::history::History;
use shell::executor;
use shell::parser;
use io::raw_io;

fn main() {
    let mut history = History::new();
    history.load_history("history.txt");

    let original_termios = raw_io::enable_raw_mode();

    loop {
        history.push(String::from(""));
        io::prompt::print_prompt("");

        let input = raw_io::read_input(&mut history);
        let input = input.trim();

        if input == "exit" || input == "__EOF__" {
            history.save_history("history.txt");
            break;
        }

        let commands = parser::parse_input(input);
        executor::execute_command(commands);
    }

    raw_io::disable_raw_mode(&original_termios);
    println!("Bye!");
}
