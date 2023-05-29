pub struct PrimeImplicantReducer {
    variables_context: VariablesContext,
    terms: Vec<String>,
    prime_implicants: Vec<String>,
}

pub struct VariablesContext {
    variables: Vec<String>,
}

impl PrimeImplicantReducer {
    pub fn new(variables: &Vec<String>, terms: &Vec<String>) -> Self {
        PrimeImplicantReducer {
            variables_context: VariablesContext { variables: variables.to_vec() },
            terms: terms.to_vec(),
            prime_implicants: Vec::new(),
        }
    }


    pub fn show_variables(&self) {
        println! {"Parsed variables: {:?}", self.variables_context.variables}
    }

    pub fn reduce_terms(&mut self) {
        // Perform reduction of terms into prime implicants
        // Implementation logic goes here
    }

    pub fn get_prime_implicants(&self) -> &Vec<String> {
        &self.prime_implicants
    }
}
