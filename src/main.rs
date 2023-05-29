use std::process::exit;

mod util {
    pub mod file_reader;
    pub mod logger;
}

mod expression_data;
mod karnaugh;
mod prime_implicant_reducer;

//use karnaugh::Karnaugh;
use prime_implicant_reducer::PrimeImplicantReducer;
use util::file_reader::FileReader;
use util::logger::Logger;

fn main() {
    //let logger_instance = Logger::new();
    //logger_instance.show_menu();
    //logger_instance.process_input(">> ");
    //logger_instance.log(&format!("You entered: {}\n", logger_instance.get_input()));

    //let mut karnaugh_instance = Karnaugh::new();
    //karnaugh_instance.assign_map(&logger_instance, logger_instance.get_input());

    let mut file_reader_instance = FileReader::new();
    let file_path = String::from("data/test1.txt");

    if let Err(err) = file_reader_instance.read_file(file_path) {
        println!("Error reading file: {}", err);
    } else {
        //file_reader_instance.expression_data.show_expression();
        //file_reader_instance.expression_data.show_variables();
        //file_reader_instance.expression_data.show_terms();
        let prime_implicant_reducer = PrimeImplicantReducer::new(
            file_reader_instance.expression_data.get_variables(),
            file_reader_instance.expression_data.get_terms(),
        );

        prime_implicant_reducer.show_variables()
    }

    exit(0);
}
