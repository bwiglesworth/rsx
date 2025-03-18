use std::collections::HashMap;

pub struct StyleCompiler {
    rules: Vec<CompiledRule>,
    scope: String,
}

impl StyleCompiler {
    pub fn new(scope: &str) -> Self {
        Self {
            rules: Vec::new(),
            scope: scope.to_string(),
        }
    }

    pub fn compile(&mut self, parsed_rules: Vec<StyleRule>) -> String {
        for rule in parsed_rules {
            let compiled = self.compile_rule(&rule);
            self.rules.push(compiled);
        }

        self.generate_css()
    }

    fn compile_rule(&self, rule: &StyleRule) -> CompiledRule {
        let selector = format!(".{} {}", self.scope, rule.selector);
        CompiledRule {
            selector,
            properties: rule.properties.clone(),
        }
    }

    fn generate_css(&self) -> String {
        self.rules.iter()
            .map(|rule| rule.to_css())
            .collect::<Vec<_>>()
            .join("\n")
    }
}
