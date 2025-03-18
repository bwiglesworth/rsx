mod card;
pub use card::Card;

use std::any::Any;

pub trait Component {
    fn render(&self) -> String;
    fn update(&mut self) -> bool;
    fn mounted(&self) {}
    fn unmounted(&self) {}
}

pub struct ComponentBuilder<T> {
    component: T,
    children: Vec<Box<dyn Component>>,
}

impl<T: Component> ComponentBuilder<T> {
    pub fn new(component: T) -> Self {
        Self {
            component,
            children: Vec::new(),
        }
    }

    pub fn add_child<C: Component + 'static>(&mut self, child: C) -> &mut Self {
        self.children.push(Box::new(child));
        self
    }

    pub fn build(self) -> T {
        self.component
    }
}