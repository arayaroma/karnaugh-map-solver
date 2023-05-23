pub struct Karnaugh {
    rows: u32,
    cols: u32,
    is_four_variable: bool,
}

impl Karnaugh {
    pub fn new() -> Self {
        Karnaugh {
            rows: 0,
            cols: 0,
            is_four_variable: false,
        }
    }

    pub fn is_four_variable(&mut self, message: &str) -> bool {
        if message == "4" {
            self.is_four_variable = true;
            true
        } else {
            false
        }
    }

    pub fn create_four_variable_map(&self) -> String {
        let mut karnaugh_map = String::new();
        if self.is_four_variable {
            karnaugh_map = String::from(
                "AB\nCD | 00 | 01 | 11 | 10 |\n\
                -----------------------|\n\
                00 |    |    |    |    |\n\
                -----------------------|\n\
                01 |    |    |    |    |\n\
                -----------------------|\n\
                11 |    |    |    |    |\n\
                -----------------------|\n\
                10 |    |    |    |    |\n",
            );
        }
        karnaugh_map
    }

    pub fn create_five_variable_map(&self) -> String {
        let mut karnaugh_map = String::new();
        if !self.is_four_variable {
            karnaugh_map = String::from(
                "ABC\nDE | 000 | 001 | 011 | 010 | 110 | 111 | 101 | 100 |\n\
                ---------------------------------------------------|\n\
                00 |     |     |     |     |     |     |     |     |\n\
                ---------------------------------------------------|\n\
                01 |     |     |     |     |     |     |     |     |\n\
                ---------------------------------------------------|\n\
                11 |     |     |     |     |     |     |     |     |\n\
                ---------------------------------------------------|\n\
                10 |     |     |     |     |     |     |     |     |\n",
            );
        }
        karnaugh_map
    }
}
