use std::any::Any;

pub trait State: Any {
    fn clone_state(&self) -> Box<dyn State>;
}

impl<T: Clone + 'static> State for T {
    fn clone_state(&self) -> Box<dyn State> {
        Box::new(self.clone())
    }
}
