use std::collections::HashMap;

pub struct FourVariableMap {
    matrix: Vec<Vec<(usize, usize)>>,
    matrix_map: HashMap<String, (usize, usize)>,
    rows: usize,
    cols: usize,
    miniterms: Vec<String>,
    gray_code: Vec<String>,
}

impl FourVariableMap {
    pub fn new() -> Self {
        let mut instance = FourVariableMap {
            matrix: Vec::new(),
            matrix_map: HashMap::new(),
            rows: 4,
            cols: 4,
            miniterms: Vec::new(),
            gray_code: Vec::new(),
        };
        instance.gray_code = instance.populate_gray_code();
        instance.miniterms = instance.populate_miniterms();
        instance.populate_matrix();
        instance.populate_matrix_map();
        instance
    }

    #[allow(dead_code)]
    pub fn get_matrix(&self) -> &Vec<Vec<(usize, usize)>> {
        &self.matrix
    }

    fn populate_gray_code(&mut self) -> Vec<String> {
        self.gray_code = vec!["00", "01", "11", "10"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        self.gray_code.clone()
    }

    fn populate_miniterms(&mut self) -> Vec<String> {
        self.miniterms = vec![
            "0",
            "1",
            "3",
            "2",
            "4",
            "5",
            "7",
            "6",
            "12",
            "13",
            "15",
            "14",
            "8",
            "9",
            "11",
            "10"
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        self.miniterms.clone()
    }

    fn populate_matrix(&mut self) {
        for i in 0..self.rows {
            let mut row = Vec::new();
            for j in 0..self.cols {
                row.push((i, j));
            }
            self.matrix.push(row);
        }
    }

    #[allow(dead_code)]
    pub fn show_populated_matrix(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn populate_matrix_map(&mut self) {
        for (index, miniterm) in self.miniterms.iter().enumerate() {
            let row = index / 4;
            let col = index % 4;
            self.matrix_map.insert(miniterm.to_string(), (row, col));
        }
    }

    #[allow(dead_code)]
    pub fn show_populated_matrix_map(&self) {
        for (miniterm, position) in &self.matrix_map {
            println!("{}={:?}", miniterm, position);
        }
    }

    #[allow(dead_code)]
    pub fn get_miniterm_position(&self, miniterm: &String) {
        println!("{:?}", self.matrix_map.get(miniterm));
    }

    pub fn print_matrix(&self, variables: &Vec<String>, miniterms: &Vec<String>) {
        let format_variables = variables.join("");
        let format_gray = self.gray_code.join("  ");
        let header = format!("{:<4} {}", format_variables, format_gray);
        println!("{}", header);

        for variable_value in &self.gray_code {
            let row = self.gray_code
                .iter()
                .position(|x| x == variable_value)
                .unwrap();
            print!("{:<4} ", variable_value);

            for col in 0..self.cols {
                let miniterm = &self.miniterms[(row * self.cols + col) as usize];

                if miniterms.contains(&miniterm) {
                    print!("{:<4}", "1");
                } else {
                    print!("{:<4}", "0");
                }
            }
            println!();
        }
    }
}
