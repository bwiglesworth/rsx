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

impl Component for Counter {
    type State = CounterState;

    fn render(&self) -> String {
        format!(
            r#"<div class="counter">
                <h2>Counter: {}</h2>
                <button onclick="increment()">+</button>
                <button onclick="decrement()">-</button>
                <div class="counter-info">Updates: {}</div>
            </div>"#,
            self.state.count,
            self.state.updates
        )
    }

    fn get_state(&self) -> &Self::State {
        &self.state
    }

    fn set_state(&mut self, state: Self::State) {
        self.state = state;
    }

    fn init(&mut self) {
        println!("Counter initialized with value: {}", self.state.count);
    }

    fn before_mount(&mut self) {
        self.state.updates = 0;
    }

    fn before_update(&mut self) -> bool {
        self.state.updates += 1;
        true
    }

    fn updated(&mut self) {
        println!("Counter updated to: {}", self.state.count);
    }
}