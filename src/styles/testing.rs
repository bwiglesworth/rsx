use super::*;

pub struct StyleTestHarness {
    registry: StyleRegistry,
    validator: StyleValidator,
    compiler: StyleCompiler,
}

impl StyleTestHarness {
    pub fn new() -> Self {
        Self {
            registry: StyleRegistry::new(),
            validator: StyleValidator::new(),
            compiler: StyleCompiler::new("test"),
        }
    }

    pub fn test_style(&mut self, input: &str) -> TestResult {
        let validated = self.validator.validate(input);
        let compiled = self.compiler.compile(input);
        
        TestResult {
            input: input.to_string(),
            validation: validated,
            compilation: compiled,
        }
    }
}
