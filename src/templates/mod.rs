use crate::Style;

pub struct AppTemplate {
    pub content: String,
    pub styles: Vec<Style>
}

impl AppTemplate {
    pub fn new(content: &str) -> Self {
        AppTemplate {
            content: content.to_string(),
            styles: Vec::new()
        }
    }

    pub fn add_styles(&mut self, style: Style) {
        self.styles.push(style);
    }

    pub fn generate(&self) -> String {
        let styles = self.styles
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"<!DOCTYPE html>
            <html>
                <head>
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