use super::Component;

#[derive(Clone)]
pub struct CardState {
    pub title: String,
    pub content: String,
}

pub struct Card {
    pub state: CardState,
    pub children: Vec<Box<dyn Component<State = CardState>>>,
}
impl Component for Card {
    type State = CardState;

    fn render(&self) -> String {
        format!(
            r#"<div class="card">
                <h2>{}</h2>
                <div class="content">{}</div>
                {}</div>"#,
            self.state.title,
            self.state.content,
            self.children.iter()
                .map(|child| child.render())
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }

    fn updated(&mut self) {}
}