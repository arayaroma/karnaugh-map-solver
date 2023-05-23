use std::fs::File;
use std::io::Result;
use std::io::{BufRead, BufReader};

pub struct FileReader {
    file_path: String,
    expression: String,
    variables: Vec<String>,
}

impl FileReader {
    /// Creates a new instance of `FileReader`.
    pub fn new() -> Self {
        FileReader {
            file_path: String::new(),
            expression: String::new(),
            variables: Vec::new(),
        }
    }

    /// Returns a reference to the variables extracted from the expression.
    pub fn get_variables(&self) -> &Vec<String> {
        &self.variables
    }

    /// Sets the file path for reading.
    fn set_file_path(&mut self, file_path: String) {
        self.file_path = file_path;
    }

    /// Reads the contents of the specified file and stores the last line as the expression.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The path to the file to be read.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a vector of lines read from the file if successful,
    /// or an error if the file cannot be opened or read.
    pub fn read_file(&mut self, file_path: String) -> Result<Vec<String>> {
        self.set_file_path(file_path);
        let file = File::open(&self.file_path)?;
        let reader = BufReader::new(file);
        let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();
        for line in &lines {
            self.expression = line.to_owned();
        }
        Ok(lines)
    }

    /// Prints the contents of the file to the console.
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

    /// Extracts variables from the stored expression.
    ///
    /// # Returns
    ///
    /// Returns a vector of string slices representing the extracted variables.
    pub fn extract_variables(&mut self) -> Vec<&str> {
        let variables: Vec<&str> = Self::trim_variables(&self.expression);
        let variables_owned: Vec<String> = variables.iter().map(|&var| var.to_string()).collect();
        self.variables = variables_owned.clone();
        variables
    }

    /// Trims the variables from the expression string.
    ///
    /// # Arguments
    ///
    /// * `expression` - The expression string containing the variables.
    ///
    /// # Returns
    ///
    /// Returns a vector of string slices representing the trimmed variables.
    fn trim_variables(expression: &str) -> Vec<&str> {
        if let Some(parentheses_start) = expression.find('(') {
            if let Some(parentheses_end) = expression.find(')') {
                let variables_str = &expression[parentheses_start + 1..parentheses_end];
                return variables_str.split(',').map(|var| var.trim()).collect();
            }
        }
        Vec::new()
    }
}
