use super::Component;

#[derive(Clone, Default)]
pub struct CardState {
    // Add any state fields here
}

pub struct Card {
    title: String,
    content: String,
    children: Vec<Box<dyn Component<State = CardState>>>,
    state: CardState,
}

impl Component for Card {
    type State = CardState;

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

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }

    fn updated(&mut self) {}
}

impl Card {
    pub fn new(title: String, content: String, children: Vec<Box<dyn Component<State = CardState>>>) -> Self {
        Self {
            title,
            content,
            children,
            state: CardState::default(),
        }
    }
}