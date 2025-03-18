use std::sync::Arc;
use tokio::sync::Mutex;

mod lifecycle;
mod counter;
mod card;

pub use lifecycle::{OnMount, LifecycleState};
pub use counter::{Counter, CounterState};
pub use card::{Card, CardState};

pub trait Component: Sized {
    type Props: Default;

    fn render(&self) -> String;
    
    fn with_props(props: Self::Props) -> ComponentBuilder<Self>;
    
    fn children(&self) -> &[Box<dyn Component>];
}

#[derive(Default)]
pub struct Props {
    pub class_name: Option<String>,
    pub style: Option<Style>,
    pub on_click: Option<Box<dyn Fn() -> ()>>,
}pub trait Mountable: Component + OnMount {
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