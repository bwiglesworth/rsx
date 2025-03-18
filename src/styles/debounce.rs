use tokio::time::{Duration, Instant, sleep};
use tokio::sync::mpsc;

pub struct StyleDebouncer {
    delay: Duration,
    last_update: Instant,
    sender: mpsc::Sender<StyleUpdate>,
    pending: Option<StyleUpdate>,
}

impl StyleDebouncer {
    pub fn new(delay: Duration, sender: mpsc::Sender<StyleUpdate>) -> Self {
        Self {
            delay,
            last_update: Instant::now(),
            sender,
            pending: None,
        }
    }

    pub async fn update(&mut self, update: StyleUpdate) {
        self.pending = Some(update);
        
        if self.last_update.elapsed() >= self.delay {
            self.flush().await;
        } else {
            sleep(self.delay - self.last_update.elapsed()).await;
            self.flush().await;
        }
    }

    async fn flush(&mut self) {
        if let Some(update) = self.pending.take() {
            self.sender.send(update).await.ok();
            self.last_update = Instant::now();
        }
    }
}
