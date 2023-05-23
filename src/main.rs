use std::process::exit;

mod file_reader;
mod karnaugh;
mod logger;

use file_reader::FileReader;
use karnaugh::Karnaugh;
use logger::Logger;

fn main() {
    let logger_instance = Logger::new();
    //logger_instance.show_menu();
    //logger_instance.process_input(">> ");
    //logger_instance.log(&format!("You entered: {}\n", logger_instance.get_input()));

    //let mut karnaugh_instance = Karnaugh::new();
    //karnaugh_instance.assign_map(&logger_instance, logger_instance.get_input());

    let mut file_reader_instance = FileReader::new();
    if let Err(err) = file_reader_instance.read_file(String::from("data/test1.txt")) {
        println!("Error reading file: {}", err);
    } else {
        file_reader_instance.log_file();
    }
    //logger_instance.log(file_reader_instance.extract_variables().join(", ").as_str());
    //logger_instance.log(file_reader_instance.get_variables().join(", ").as_str());

    exit(0);
}
