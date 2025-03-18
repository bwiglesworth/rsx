use std::sync::Arc;
use parking_lot::RwLock;

pub struct ComponentRegistry {
    factory: Arc<RwLock<ComponentFactory>>,
    styles: Arc<RwLock<StyleRegistry>>,
}

impl ComponentRegistry {
    pub fn global() -> &'static Self {
        static REGISTRY: once_cell::sync::Lazy<ComponentRegistry> = 
            once_cell::sync::Lazy::new(|| ComponentRegistry::new());
        ®ISTRY
    }

    pub fn register_component<T: Component + 'static>(&self, name: &str) {
        let mut factory = self.factory.write();
        factory.register::<T>(name);
    }

    pub fn create_component(&self, name: &str) -> Option<Box<dyn Component>> {
        let factory = self.factory.read();
        factory.create_by_name(name)
    }
}
