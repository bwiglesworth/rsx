use std::collections::HashMap;

pub struct StyleInheritance {
    inheritance_map: HashMap<String, Vec<String>>,
    computed_styles: HashMap<String, ComputedStyle>,
}

impl StyleInheritance {
    pub fn new() -> Self {
        Self {
            inheritance_map: HashMap::new(),
            computed_styles: HashMap::new(),
        }
    }

    pub fn inherit(&mut self, component: &str, parent: &str) {
        self.inheritance_map
            .entry(parent.to_string())
            .or_default()
            .push(component.to_string());
    }

    pub fn compute_inherited_styles(&mut self, root: &str) -> ComputedStyle {
        let mut computed = ComputedStyle::new();
        self.compute_recursive(root, &mut computed);
        computed
    }

    fn compute_recursive(&self, component: &str, computed: &mut ComputedStyle) {
        if let Some(children) = self.inheritance_map.get(component) {
            for child in children {
                self.compute_recursive(child, computed);
            }
        }
    }
}
