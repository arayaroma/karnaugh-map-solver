pub struct ExpressionData {
    variables: Vec<String>,
    terms: Vec<String>,
}

impl ExpressionData {
    pub fn new() -> Self {
        ExpressionData {
            variables: Vec::new(),
            terms: Vec::new(),
        }
    }

    pub fn add_variable(&mut self, variable: String) {
        self.variables.push(variable);
    }

    pub fn add_term(&mut self, term: String) {
        self.terms.push(term);
    }

    pub fn get_variables(&self) -> &Vec<String> {
        &self.variables
    }

    pub fn get_terms(&self) -> &Vec<String> {
        &self.terms
    }
}
