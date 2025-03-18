pub struct StyleComposer {
    layers: Vec<CompositionLayer>,
    mixins: HashMap<String, StyleMixin>,
}

impl StyleComposer {
    pub fn new() -> Self {
        Self {
            layers: Vec::new(),
            mixins: HashMap::new(),
        }
    }

    pub fn compose(&mut self, base: StyleRule) -> ComposedStyle {
        let mut result = base;
        
        for layer in &self.layers {
            match layer {
                CompositionLayer::Mixin(name) => {
                    if let Some(mixin) = self.mixins.get(name) {
                        result = self.apply_mixin(&result, mixin);
                    }
                },
                CompositionLayer::Override(rules) => {
                    result = self.apply_overrides(&result, rules);
                }
            }
        }
        
        ComposedStyle::new(result)
    }
}
