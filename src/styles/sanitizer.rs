pub struct StyleSanitizer {
    allowed_properties: HashSet<String>,
    allowed_values: HashMap<String, Vec<String>>,
    sanitization_rules: Vec<Box<dyn SanitizationRule>>,
}

impl StyleSanitizer {
    pub fn new() -> Self {
        Self {
            allowed_properties: Self::default_allowed_properties(),
            allowed_values: Self::default_allowed_values(),
            sanitization_rules: vec![
                Box::new(UrlSanitizer::new()),
                Box::new(ColorSanitizer::new()),
                Box::new(UnitSanitizer::new()),
            ],
        }
    }

    pub fn sanitize(&self, style: StyleRule) -> StyleRule {
        let mut sanitized = style;
        
        for rule in &self.sanitization_rules {
            sanitized = rule.apply(sanitized);
        }
        
        sanitized
    }
}
