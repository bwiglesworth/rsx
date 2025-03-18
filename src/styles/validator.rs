use std::collections::HashSet;

pub struct StyleValidator {
    valid_properties: HashSet<String>,
    valid_units: HashSet<String>,
    errors: Vec<ValidationError>,
}

impl StyleValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            valid_properties: HashSet::new(),
            valid_units: HashSet::new(),
            errors: Vec::new(),
        };
        
        validator.initialize_valid_properties();
        validator.initialize_valid_units();
        validator
    }

    pub fn validate(&mut self, style: &StyleRule) -> ValidationResult {
        self.errors.clear();
        
        self.validate_selector(&style.selector);
        self.validate_properties(&style.properties);
        
        if self.errors.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(self.errors.clone())
        }
    }
}
