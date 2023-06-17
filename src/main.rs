use std::process::exit;

mod util {
    pub mod file_reader;
    pub mod logger;
}

mod expression_data;
mod prime_implicant_reducer;
mod four_variable_map;

use prime_implicant_reducer::PrimeImplicantReducer;
use util::file_reader::FileReader;
use four_variable_map::FourVariableMap;

fn main() {
    //let logger_instance = Logger::new();
    //logger_instance.show_menu();
    //logger_instance.process_input(">> ");
    //logger_instance.log(&format!("You entered: {}\n", logger_instance.get_input()));

    let mut file_reader_instance = FileReader::new();
    let file_path = String::from("data/test1.txt");

    if let Err(err) = file_reader_instance.read_file(file_path) {
        println!("Error reading file: {}", err);
    } else {
        let prime_implicant_reducer = PrimeImplicantReducer::new(
            file_reader_instance.expression_data.get_variables(),
            file_reader_instance.expression_data.get_terms()
        );

        prime_implicant_reducer.show_variables();
    }

    let four_variable_map = FourVariableMap::new();
    four_variable_map.show_populated_matrix();
    four_variable_map.show_populated_matrix_map();

    exit(0);
}
