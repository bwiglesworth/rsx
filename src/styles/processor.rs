use tokio::sync::mpsc;
use std::sync::Arc;

pub struct StyleProcessor {
    receiver: mpsc::Receiver<StyleUpdate>,
    cache: Arc<StyleCache>,
    optimizer: Arc<StyleOptimizer>,
}

impl StyleProcessor {
    pub fn new(cache: Arc<StyleCache>, optimizer: Arc<StyleOptimizer>) -> (Self, mpsc::Sender<StyleUpdate>) {
        let (sender, receiver) = mpsc::channel(100);
        
        (Self {
            receiver,
            cache,
            optimizer,
        }, sender)
    }

    pub async fn run(&mut self) {
        while let Some(update) = self.receiver.recv().await {
            let optimized = self.optimizer.optimize(update.style.clone());
            self.cache.insert(update.path.clone(), optimized);
            
            // Notify components of the processed update
            StyleBroadcaster::global().broadcast(StyleUpdate::Processed {
                path: update.path,
                style: self.cache.get(&update.path).unwrap(),
            });
        }
    }
}
