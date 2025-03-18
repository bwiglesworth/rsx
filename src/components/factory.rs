use std::any::TypeId;
use std::collections::HashMap;

pub struct ComponentFactory {
    components: HashMap<TypeId, Box<dyn Fn() -> Box<dyn Component>>>,
}

impl ComponentFactory {
    pub fn new() -> Self {
        Self {
            components: HashMap::new()
        }
    }

    pub fn register<T: Component + 'static>(&mut self, creator: fn() -> T) {
        self.components.insert(
            TypeId::of::<T>(),
            Box::new(move || Box::new(creator()))
        );
    }

    pub fn create<T: Component + 'static>(&self) -> Option<Box<dyn Component>> {
        self.components
            .get(&TypeId::of::<T>())
            .map(|creator| creator())
    }
}
