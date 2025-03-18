use crate::style;

pub struct Button {
    style: Style,
}

impl Button {
    pub fn new() -> Self {
        let style = style! {
            "button" => {
                "background-color" => "#0070f3",
                "color" => "white",
                "border" => "none",
                "padding" => "0.5rem 1rem",
                "border-radius" => "4px",
                "cursor" => "pointer"
            },
            "button:hover" => {
                "background-color" => "#0051cc"
            }
        };

        Button { style }
    }

    pub fn render(&self) -> String {
        format!(
            r#"
            <style>{}</style>
            <button class="button">Click me</button>
            "#,
            self.style
        )
    }
}
