use super::Component;

pub struct Card {
    pub title: String,
    pub content: String,
    pub children: Vec<Box<dyn Component>>,
}
impl Component for Card {
    fn render(&self) -> String {
        format!(
            r#"<div class="card">
                <h2>{}</h2>
                <div class="content">{}</div>
                {}</div>"#,
            self.title,
            self.content,
            self.children.iter()
                .map(|child| child.render())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn update(&mut self) -> bool {
        true
    }
}