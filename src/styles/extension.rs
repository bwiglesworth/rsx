pub struct StyleExtender {
    extensions: HashMap<String, StyleExtension>,
    inheritance_chain: Vec<String>,
}

impl StyleExtender {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
            inheritance_chain: Vec::new(),
        }
    }

    pub fn extend(&mut self, base: &str, extension: StyleExtension) -> ExtendedStyle {
        self.inheritance_chain.push(base.to_string());
        
        let mut result = self.get_base_style(base);
        
        for modifier in extension.modifiers {
            result = self.apply_modifier(result, modifier);
        }
        
        ExtendedStyle {
            base: base.to_string(),
            modified: result,
            chain: self.inheritance_chain.clone(),
        }
    }
}
