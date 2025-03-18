use super::Component;

#[derive(Clone)]
pub struct CounterState {
    pub count: i32,
    pub updates: i32,
}

pub struct Counter {
    pub state: CounterState,
    pub children: Vec<Box<dyn Component<State = CounterState>>>,
}

impl Counter {
    pub fn new() -> Self {
        Self {
            state: CounterState { count: 0, updates: 0 },
            children: Vec::new(),
        }
    }
}

impl Component for Counter {
    type State = CounterState;

    fn render(&self) -> String {
        format!("<div>Count: {}</div>", self.state.count)
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }
}