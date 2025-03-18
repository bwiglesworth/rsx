#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_style_creation() {
        let style = style! {
            "button" => {
                "color" => "blue",
                "padding" => "10px"
            }
        };
        
        let output = style.to_string();
        assert!(output.contains("button {"));
        assert!(output.contains("color: blue;"));
        assert!(output.contains("padding: 10px;"));
    }

    #[test]
    fn test_multiple_rules() {
        let style = style! {
            ".container" => {
                "max-width" => "800px",
                "margin" => "0 auto"
            },
            ".container:hover" => {
                "background" => "#f0f0f0"
            }
        };

        let output = style.to_string();
        assert!(output.contains(".container {"));
        assert!(output.contains(".container:hover {"));
        assert!(output.contains("max-width: 800px;"));
        assert!(output.contains("background: #f0f0f0;"));
    }

    #[test]
    fn test_template_style_integration() {
        let mut template = AppTemplate::new("Test App");
        let style = style! {
            "body" => {
                "margin" => "0",
                "padding" => "20px"
            }
        };

        template.add_styles(style);
        let html = template.generate();
        
        assert!(html.contains("<style>"));
        assert!(html.contains("body {"));
        assert!(html.contains("margin: 0;"));
    }
}
