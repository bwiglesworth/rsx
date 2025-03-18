use std::sync::Arc;

pub struct ComponentStyles {
    registry: Arc<StyleRegistry>,
    class_names: Vec<String>,
}

impl ComponentStyles {
    pub fn new(class_names: &str) -> Self {
        Self {
            registry: StyleRegistry::global(),
            class_names: class_names.split_whitespace()
                .map(String::from)
                .collect(),
        }
    }

    pub fn apply(&self, component: &dyn Component) -> String {
        let mut styles = Vec::new();
        for class_name in &self.class_names {
            if let Some(style) = self.registry.get(class_name) {
                styles.push(style.to_string());
            }
        }
        styles.join("\n")
    }
}
