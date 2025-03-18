use std::sync::Arc;
use tokio::sync::broadcast;

pub trait StyleSubscriber: Send + Sync {
    fn handle_style_update(&self, update: &StyleUpdate);
    fn get_subscribed_classes(&self) -> Vec<String>;
}

pub struct ComponentStyleSubscriber {
    id: String,
    receiver: broadcast::Receiver<StyleUpdate>,
    classes: Arc<RwLock<Vec<String>>>,
}

impl ComponentStyleSubscriber {
    pub fn new(id: String, classes: Vec<String>) -> Self {
        let receiver = StyleBroadcaster::global().subscribe();
        Self {
            id,
            receiver,
            classes: Arc::new(RwLock::new(classes)),
        }
    }

    pub async fn listen(&mut self) {
        while let Ok(update) = self.receiver.recv().await {
            if self.should_update(&update) {
                self.handle_style_update(&update);
            }
        }
    }
}
