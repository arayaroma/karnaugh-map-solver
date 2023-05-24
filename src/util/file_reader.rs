use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

use crate::expression_data::ExpressionData;

pub struct FileReader {
    file_path: String,
    pub expression_data: ExpressionData,
}

impl FileReader {
    pub fn new() -> Self {
        FileReader {
            file_path: String::new(),
            expression_data: ExpressionData::new(),
        }
    }

    fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
    }

    pub fn read_file(&mut self, file_path: String) -> Result<Vec<String>> {
        self.set_file_path(file_path);
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        self.expression_data.extract_expression(lines.clone());
        Ok(lines)
    }

    #[allow(dead_code)]
    pub fn log_file(&self) {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            for line in reader.lines().filter_map(Result::ok) {
                println!("{}", line);
            }
        } else {
            println!("Error opening file");
        }
    }
}
