pub struct StyleResolver {
    rules: Vec<StyleRule>,
    specificity_map: HashMap<String, u32>,
}

impl StyleResolver {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            specificity_map: HashMap::new(),
        }
    }

    pub fn resolve_conflicts(&mut self) -> Vec<ResolvedStyle> {
        let mut resolved = Vec::new();
        let mut property_map: HashMap<String, Vec<(StyleRule, u32)>> = HashMap::new();

        // Group rules by property
        for rule in &self.rules {
            let specificity = self.calculate_specificity(&rule.selector);
            for (property, value) in &rule.properties {
                property_map
                    .entry(property.clone())
                    .or_default()
                    .push((rule.clone(), specificity));
            }
        }

        // Resolve conflicts for each property
        for (property, rules) in property_map {
            let winner = self.determine_winner(rules);
            resolved.push(winner);
        }

        resolved
    }
}
