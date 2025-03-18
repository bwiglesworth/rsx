use tokio::time::{Duration, Instant};
use tokio::sync::mpsc;

pub struct StyleThrottler {
    rate_limit: Duration,
    last_update: Instant,
    sender: mpsc::Sender<StyleUpdate>,
    buffer: Vec<StyleUpdate>,
}

impl StyleThrottler {
    pub fn new(rate_limit: Duration, sender: mpsc::Sender<StyleUpdate>) -> Self {
        Self {
            rate_limit,
            last_update: Instant::now(),
            sender,
            buffer: Vec::new(),
        }
    }

    pub async fn process(&mut self, update: StyleUpdate) {
        let now = Instant::now();
        
        if now.duration_since(self.last_update) >= self.rate_limit {
            self.send_update(update).await;
            self.last_update = now;
        } else {
            self.buffer.push(update);
        }
    }

    async fn send_update(&mut self, update: StyleUpdate) {
        self.sender.send(update).await.ok();
    }
}
