impl AppTemplate {
    pub fn add_styles(&mut self, styles: Style) {
        self.styles.push(styles);
    }

    pub fn generate(&self) -> String {
        let styles = self.styles
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <style>{}</style>
                </head>
                <body>
                    {}
                </body>
            </html>
            "#,
            styles,
            self.content
        )
    }
}
