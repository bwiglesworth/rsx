use notify::{Watcher, RecursiveMode, Event};
use tokio::sync::broadcast;

pub struct StyleWatcher {
    sender: broadcast::Sender<StyleUpdate>,
    _watcher: notify::RecommendedWatcher,
}

impl StyleWatcher {
    pub fn new(style_dir: &str) -> Self {
        let (sender, _) = broadcast::channel(100);
        let sender_clone = sender.clone();

        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                let update = StyleUpdate::from_event(event);
                sender_clone.send(update).ok();
            }
        }).unwrap();

        watcher.watch(style_dir.as_ref(), RecursiveMode::Recursive).unwrap();

        Self {
            sender,
            _watcher: watcher,
        }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<StyleUpdate> {
        self.sender.subscribe()
    }
}
