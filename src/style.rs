#[derive(Clone)]
pub struct Style {
    selector: String,
    properties: Vec<(String, String)>,
}

impl Style {
    pub fn new(selector: &str) -> Self {
        Style {
            selector: selector.to_string(),
            properties: Vec::new(),
        }
    }

    pub fn property(mut self, name: &str, value: &str) -> Self {
        self.properties.push((name.to_string(), value.to_string()));
        self
    }
}
