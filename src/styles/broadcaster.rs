use tokio::sync::broadcast;
use std::sync::Arc;

pub struct StyleBroadcaster {
    sender: broadcast::Sender<StyleUpdate>,
    components: Arc<RwLock<HashMap<String, Box<dyn StyleSubscriber>>>>,
}

impl StyleBroadcaster {
    pub fn global() -> &'static Self {
        static BROADCASTER: once_cell::sync::Lazy<StyleBroadcaster> = 
            once_cell::sync::Lazy::new(|| StyleBroadcaster::new());
        &BROADCASTER
    }

    pub fn broadcast(&self, update: StyleUpdate) {
        let components = self.components.read();
        for component in components.values() {
            component.handle_style_update(&update);
        }
    }

    pub fn subscribe(&self, id: String, component: Box<dyn StyleSubscriber>) {
        let mut components = self.components.write();
        components.insert(id, component);
    }
}
