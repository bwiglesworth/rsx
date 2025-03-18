pub struct StyleNormalizer {
    vendor_prefixes: Vec<String>,
    shorthand_properties: HashMap<String, Vec<String>>,
    value_aliases: HashMap<String, String>,
}

impl StyleNormalizer {
    pub fn new() -> Self {
        Self {
            vendor_prefixes: vec!["-webkit-".into(), "-moz-".into(), "-ms-".into(), "-o-".into()],
            shorthand_properties: Self::init_shorthand_properties(),
            value_aliases: Self::init_value_aliases(),
        }
    }

    pub fn normalize(&self, style: StyleRule) -> StyleRule {
        let mut normalized = style;
        normalized = self.expand_shorthand_properties(normalized);
        normalized = self.normalize_vendor_prefixes(normalized);
        normalized = self.resolve_value_aliases(normalized);
        normalized
    }
}
