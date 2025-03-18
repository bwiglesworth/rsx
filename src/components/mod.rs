use std::sync::Arc;
use tokio::sync::Mutex;

mod lifecycle;
mod counter;
mod card;

pub use lifecycle::{OnMount, LifecycleState};
pub use counter::{Counter, CounterState};
pub use card::{Card, CardState};

pub trait Component {
    type State;
    fn render(&self) -> String;
    fn get_state(&self) -> &Self::State;
    fn set_state(&mut self, state: Self::State);
    
    fn init(&mut self) {}
    fn before_mount(&mut self) {}
    fn before_update(&mut self) -> bool { true }
    fn updated(&mut self) {}
}

pub trait Mountable: Component + OnMount {
    fn mount(&self) {
        let future = self.on_mount();
        tokio::spawn(future);
    }
}

pub struct ComponentBuilder<T: Component> {
    component: T,
    lifecycle_state: Arc<Mutex<LifecycleState>>,
}

impl<T: Component> ComponentBuilder<T> {
    pub fn new(component: T) -> Self {
        Self {
            component,
            lifecycle_state: Arc::new(Mutex::new(LifecycleState::BeforeMount)),
        }
    }

    pub fn build(self) -> T {
        let state = self.lifecycle_state.clone();
        tokio::spawn(async move {
            let mut lock = state.lock().await;
            *lock = LifecycleState::Mounted;
        });
        self.component
    }

    pub fn get_lifecycle_state(&self) -> Arc<Mutex<LifecycleState>> {
        self.lifecycle_state.clone()
    }
}