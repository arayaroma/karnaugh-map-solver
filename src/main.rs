use std::process::exit;

mod util {
    pub mod file_reader;
    pub mod logger;
}

mod expression_data;
mod prime_implicant_reducer;
mod four_variable_map;

use util::file_reader::FileReader;
use four_variable_map::FourVariableMap;

fn main() {
    let mut file_reader_instance = FileReader::new();
    let file_path = String::from("data/test1.txt");

    if let Err(err) = file_reader_instance.read_file(file_path) {
        println!("Error reading file: {}", err);
    } else {
        file_reader_instance.expression_data.show_variables();
        file_reader_instance.expression_data.show_terms();
    }

    let four_variable_map = FourVariableMap::new();
    four_variable_map.print_matrix(
        file_reader_instance.expression_data.get_variables(),
        file_reader_instance.expression_data.get_terms()
    );

    exit(0);
}
