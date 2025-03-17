#[allow(unused_imports)]
use crate::components::{Component, Props};

pub struct AppTemplate {
    title: String,
    props: Props,
}

impl AppTemplate {
    pub fn new(title: &str) -> Self {
        AppTemplate {
            title: title.to_string(),
            props: Props::default(),
        }
    }

    pub fn with_props(mut self, props: Props) -> Self {
        self.props = props;
        self
    }

    pub fn generate(&self) -> String {
        format!(
            r#"
            <!DOCTYPE html>
            <html lang="en">
            <head>
                <meta charset="UTF-8">
                <title>{}</title>
                <link rel="stylesheet" href="/styles/main.css">
            </head>
            <body>
                <div id="root"></div>
                <script src="/scripts/main.js"></script>
            </body>
            </html>
            "#,
            self.title
        )
    }
}