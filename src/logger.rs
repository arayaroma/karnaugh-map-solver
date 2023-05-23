use std::io::Write;

pub struct Logger {
    input: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            input: String::new(),
        }
    }

    pub fn log(&self, message: &str) {
        println!("{}", message);
    }

    pub fn get_input(&self) -> &str {
        &self.input
    }

    pub fn process_input(&mut self, message: &str) {
        use std::io::{stdin, stdout};
        let mut input = String::new();

        print!("{}", message);
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut input).expect("Failed to read input");

        let input = input.trim();

        match input {
            "q" => self.exit_program("Exiting program..."),
            "4" | "5" => {
                self.input = input.to_owned();
            }
            _ => {
                println!("Invalid input. Please try again.");
                self.process_input(message);
            }
        }
    }

    pub fn exit_program(&self, message: &str) {
        println!("{}", message);
        std::process::exit(0);
    }

    pub fn clear_screen(&self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
}
