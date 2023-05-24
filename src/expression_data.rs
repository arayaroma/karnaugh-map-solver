pub struct ExpressionData {
    expression: String,
    variables: Vec<String>,
    terms: Vec<String>,
}

impl ExpressionData {
    pub fn new() -> Self {
        ExpressionData {
            expression: String::new(),
            variables: Vec::new(),
            terms: Vec::new(),
        }
    }

    pub fn show_expression(&self) {
        println!("Expression: {}", self.expression);
    }

    pub fn extract_expression(&mut self, lines: Vec<String>) {
        for line in &lines {
            self.expression = line.to_owned();
        }
        self.extract_variables();
        self.extract_terms();
    }

    pub fn show_variables(&self) {
        println!("Variables: {:?}", self.variables);
    }

    pub fn extract_variables(&mut self) {
        let variables: Vec<&str> = Self::trim_variables(self.get_expression());
        let variables_owned: Vec<String> = variables.iter().map(|&var| var.to_string()).collect();
        self.variables = variables_owned;
    }

    fn trim_variables(expression: &String) -> Vec<&str> {
        if let Some(parentheses_start) = expression.find('(') {
            if let Some(parentheses_end) = expression.find(')') {
                let variables_str = &expression[parentheses_start + 1..parentheses_end];
                return variables_str.split(',').map(|var| var.trim()).collect();
            }
        }
        Vec::new()
    }

    pub fn show_terms(&self) {
        println!("Terms: {:?}", self.terms);
    }

    pub fn extract_terms(&mut self) {
        let terms: Vec<&str> = Self::trim_terms(self.get_expression());
        let terms_owned: Vec<String> = terms.iter().map(|&term| term.to_string()).collect();
        self.terms = terms_owned;
    }

    pub fn trim_terms(expression: &str) -> Vec<&str> {
        if let Some(sum) = expression.find("sum(") {
            if let Some(parentheses_end) = expression.rfind(')') {
                let terms_str = &expression[sum + 4..parentheses_end];
                return terms_str.split(',').map(|term| term.trim()).collect();
            }
        }
        Vec::new()
    }

    pub fn get_expression(&self) -> &String {
        &self.expression
    }
  
}
