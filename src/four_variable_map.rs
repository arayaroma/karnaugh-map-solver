use std::collections::HashMap;

pub struct FourVariableMap {
    matrix: Vec<Vec<(u32, u32)>>,
    matrix_map: HashMap<String, (u32, u32)>,
    rows: u32,
    cols: u32,
}

impl FourVariableMap {
    pub fn new() -> Self {
        let mut instance = FourVariableMap {
            matrix: Vec::new(),
            matrix_map: HashMap::new(),
            rows: 4,
            cols: 4,
        };
        instance.populate_matrix();
        instance.populate_matrix_map();
        instance
    }

    #[allow(dead_code)]
    pub fn get_matrix(&self) -> &Vec<Vec<(u32, u32)>> {
        &self.matrix
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

    pub fn show_populated_matrix(&self) {
        for row in &self.matrix {
            println!("{:?}", row);
        }
    }

    fn populate_matrix_map(&mut self) {
        let miniterms = vec![
            "m0",
            "m1",
            "m3",
            "m2",
            "m4",
            "m5",
            "m7",
            "m6",
            "m12",
            "m13",
            "m15",
            "m14",
            "m8",
            "m9",
            "m11",
            "m10"
        ];

        for (index, miniterm) in miniterms.iter().enumerate() {
            let row = index / 4;
            let col = index % 4;
            self.matrix_map.insert(miniterm.to_string(), (row as u32, col as u32));
        }
    }

    pub fn show_populated_matrix_map(&self) {
        for (miniterm, position) in &self.matrix_map {
            println!("{}={:?}", miniterm, position);
        }
    }
}
