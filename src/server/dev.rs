use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use tokio::sync::broadcast;

pub struct DevServer {
    watcher: Box<dyn Watcher>,
    reload_tx: broadcast::Sender<()>,
}

impl DevServer {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1);
        let (watcher_tx, watcher_rx) = channel();
        
        let mut watcher = watcher(watcher_tx, Duration::from_secs(1)).unwrap();
        watcher.watch("./src", RecursiveMode::Recursive).unwrap();

        DevServer {
            watcher: Box::new(watcher),
            reload_tx: tx,
        }
    }

    pub async fn watch(&self) {
        println!("Watching for file changes...");
        // Hot reload implementation will go here
    }
}
