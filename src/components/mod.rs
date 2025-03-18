mod state;
mod card;
mod counter;

pub use state::State;
pub use card::{Card, CardState};
pub use counter::{Counter, CounterState};pub trait Component {
    type State;

    fn render(&self) -> String;
    fn get_state(&self) -> &Self::State;
    fn set_state(&mut self, state: Self::State);
    
    // Lifecycle hooks
    fn init(&mut self) {}
    fn before_mount(&mut self) {}
    fn mounted(&mut self) {}
    fn before_update(&mut self) -> bool { true }
    fn updated(&mut self) {}
    fn before_unmount(&mut self) {}
}pub struct ComponentBuilder<T: Component> {
    component: T,
    children: Vec<Box<dyn Component<State = T::State>>>,
}
impl<T: Component> ComponentBuilder<T> {
    pub fn new(component: T) -> Self {
        Self {
            component,
            children: Vec::new(),
        }
    }

    pub fn add_child<C>(&mut self, child: C) -> &mut Self 
    where 
        C: Component<State = T::State> + 'static 
    {
        self.children.push(Box::new(child));
        self
    }

    pub fn build(self) -> T {
        self.component
    }
}