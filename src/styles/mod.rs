use std::fmt;

use std::collections::HashMap;

pub struct StyleRegistry {
    styles: HashMap<String, Style>,
}

#[derive(Default)]
pub struct Style {
    rules: HashMap<String, HashMap<String, String>>,
}

impl Style {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, selector: &str) -> &mut StyleRule {
        self.rules.entry(selector.to_string())
            .or_default()
    }
}

pub struct StyleRule<'a> {
    properties: &'a mut HashMap<String, String>,
}

impl<'a> StyleRule<'a> {
    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.properties.insert(name.to_string(), value.to_string());
        self
    }
}
impl fmt::Display for Style {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for rule in &self.rules {
            writeln!(f, "{} {{", rule.selector)?;
            for (name, value) in &rule.properties {
                writeln!(f, "    {}: {};", name, value)?;
            }
            writeln!(f, "}}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AppTemplate;

    #[test]
    fn test_style_creation() {
        let mut style = Style::new();
        style.add_rule("button")
            .property("color", "blue")
            .property("padding", "10px");
        
        let output = style.to_string();
        assert!(output.contains("button {"));
        assert!(output.contains("color: blue;"));
        assert!(output.contains("padding: 10px;"));
    }

    #[test]
    fn test_template_integration() {
        let mut template = AppTemplate::new("Test");
        let mut style = Style::new();
        style.add_rule("body")
            .property("margin", "0");
        
        template.add_styles(style);
        let html = template.generate();
        assert!(html.contains("<style>"));
        assert!(html.contains("margin: 0;"));
    }
}
