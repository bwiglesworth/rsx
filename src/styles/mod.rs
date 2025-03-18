use std::fmt;

pub struct Style {
    rules: Vec<StyleRule>,
}

pub struct StyleRule {
    selector: String,
    properties: Vec<(String, String)>,
}

impl Style {
    pub fn new() -> Self {
        Style { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, selector: &str) -> &mut StyleRule {
        let rule = StyleRule {
            selector: selector.to_string(),
            properties: Vec::new(),
        };
        self.rules.push(rule);
        self.rules.last_mut().unwrap()
    }
}

impl StyleRule {
    pub fn property(&mut self, name: &str, value: &str) -> &mut Self {
        self.properties.push((name.to_string(), value.to_string()));
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
