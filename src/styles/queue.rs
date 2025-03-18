use tokio::sync::mpsc;
use std::collections::VecDeque;

pub struct StyleQueue {
    queue: VecDeque<StyleUpdate>,
    sender: mpsc::Sender<StyleUpdate>,
    batch_size: usize,
}

impl StyleQueue {
    pub fn new(sender: mpsc::Sender<StyleUpdate>, batch_size: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            sender,
            batch_size,
        }
    }

    pub async fn process_updates(&mut self) {
        while let Some(batch) = self.take_batch() {
            for update in batch {
                if let Err(e) = self.sender.send(update).await {
                    log::error!("Failed to send style update: {}", e);
                    self.queue.push_front(e.0);
                    break;
                }
            }
        }
    }

    fn take_batch(&mut self) -> Option<Vec<StyleUpdate>> {
        if self.queue.is_empty() {
            return None;
        }

        Some(self.queue.drain(..self.batch_size.min(self.queue.len())).collect())
    }
}
