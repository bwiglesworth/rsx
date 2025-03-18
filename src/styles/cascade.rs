use std::collections::BTreeMap;

pub struct StyleCascade {
    layers: BTreeMap<u32, StyleLayer>,
    current_layer: u32,
}

impl StyleCascade {
    pub fn new() -> Self {
        Self {
            layers: BTreeMap::new(),
            current_layer: 0,
        }
    }

    pub fn push_layer(&mut self) -> &mut StyleLayer {
        self.current_layer += 1;
        self.layers.entry(self.current_layer).or_insert_with(StyleLayer::new)
    }

    pub fn compute_final_styles(&self) -> ComputedStyle {
        let mut final_style = ComputedStyle::new();
        
        for layer in self.layers.values() {
            final_style.merge(layer.get_styles());
        }
        
        final_style
    }
}

pub struct StyleLayer {
    rules: Vec<StyleRule>,
    overrides: HashMap<String, String>,
}
