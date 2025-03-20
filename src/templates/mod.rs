use crate::components::Component;
use crate::styles::Style;

pub struct AppTemplate {
    content: String,
    styles: Vec<Style>,
}

impl AppTemplate {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
            styles: Vec::new(),
        }
    }

    pub fn add_styles(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn generate(&self) -> String {
        let styles = self.styles.iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>RSX App</title>
                <style>{}</style>
            </head>
            <body>
                {}
            </body>
            </html>"#,
            styles,
            self.content
        )
    }
}

impl Component for AppTemplate {
    type State = ();

    fn get_state(&self) -> &Self::State {
        &()
    }

    fn set_state(&mut self, _state: Self::State) {}

    fn render(&self) -> String {
        self.generate()
    }
}