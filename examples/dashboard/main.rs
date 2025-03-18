struct Dashboard {
    components: Vec<Box<dyn Component>>,
}

impl Component for Dashboard {
    fn render(&self) -> String {
        let mut style = Style::new();
        style.add_rule(".dashboard")
            .property("display", "grid")
            .property("grid-template-columns", "repeat(auto-fit, minmax(300px, 1fr))");

        let components = self.components.iter()
            .map(|c| c.render())
            .collect::<Vec<_>>()
            .join("\n");

        format!("<div class='dashboard'>{}{}</div>", style, components)
    }
}
