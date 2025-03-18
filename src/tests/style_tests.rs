#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_style_pipeline() {
        let mut harness = StyleTestHarness::new();
        
        let test_style = r#"
            .card {
                background: #fff;
                padding: 1rem;
                border-radius: 4px;
            }
        "#;

        let result = harness.test_style(test_style);
        assert!(result.validation.is_valid());
        assert!(result.compilation.is_some());
    }

    #[test]
    fn test_style_inheritance() {
        let mut harness = StyleTestHarness::new();
        
        let base_style = ".base { color: blue; }";
        let extended_style = ".extended { @extend .base; font-size: 16px; }";
        
        let result = harness.test_style(extended_style);
        assert!(result.validation.is_valid());
        assert!(result.compilation.unwrap().contains("color: blue"));
    }
}
