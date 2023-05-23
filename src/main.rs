use std::process::exit;

mod karnaugh;
mod logger;

use karnaugh::Karnaugh;
use logger::Logger;

fn main() {
    let mut logger = Logger::new();
    logger.log("+------------------------+");
    logger.log("|  Karnaugh Map Solver   |");
    logger.log("+------------------------+\n");
    logger.log("Enter the number of variables (4 or 5): ");
    logger.log("\nyou can exit the program by typing 'q'\n");
    logger.process_input(">> ");
    logger.log(&format!("You entered: {}\n", logger.get_input()));

    let mut karnaugh = Karnaugh::new();
    if karnaugh.is_four_variable(logger.get_input()) {
        logger.log(&karnaugh.create_four_variable_map());
    } else {
        logger.log(&karnaugh.create_five_variable_map());
    }

    exit(0);
}
