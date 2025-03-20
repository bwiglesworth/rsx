pub struct MainLayout {
    content: String,
}

impl Component for MainLayout {
    fn render(&self) -> String {
        format!("<div class='layout'>{}</div>", self.content)
    }
}
