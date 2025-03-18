use std::collections::HashMap;

pub struct StyleDiffer {
    previous: HashMap<String, CompiledStyle>,
    current: HashMap<String, CompiledStyle>,
}

impl StyleDiffer {
    pub fn new() -> Self {
        Self {
            previous: HashMap::new(),
            current: HashMap::new(),
        }
    }

    pub fn diff(&mut self, new_styles: HashMap<String, CompiledStyle>) -> Vec<StyleChange> {
        self.previous = std::mem::take(&mut self.current);
        self.current = new_styles;

        let mut changes = Vec::new();
        
        // Find modified and added styles
        for (key, style) in &self.current {
            match self.previous.get(key) {
                Some(prev_style) if prev_style != style => {
                    changes.push(StyleChange::Modified(key.clone(), style.clone()));
                }
                None => {
                    changes.push(StyleChange::Added(key.clone(), style.clone()));
                }
                _ => {}
            }
        }

        // Find removed styles
        for key in self.previous.keys() {
            if !self.current.contains_key(key) {
                changes.push(StyleChange::Removed(key.clone()));
            }
        }

        changes
    }
}
