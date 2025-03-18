#[macro_export]
macro_rules! style {
    ($($selector:expr => {
        $($prop:expr => $value:expr),* $(,)?
    }),* $(,)?) => {{
        let mut style = Style::new();
        $(
            let rule = style.add_rule($selector);
            $(
                rule.property($prop, $value);
            )*
        )*
        style
    }};
}
