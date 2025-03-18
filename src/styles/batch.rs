use tokio::time::{Duration, Instant};

pub struct StyleBatcher {
    batch: Vec<StyleUpdate>,
    last_flush: Instant,
    max_batch_size: usize,
    flush_interval: Duration,
}

impl StyleBatcher {
    pub fn new(max_batch_size: usize, flush_interval: Duration) -> Self {
        Self {
            batch: Vec::with_capacity(max_batch_size),
            last_flush: Instant::now(),
            max_batch_size,
            flush_interval,
        }
    }

    pub fn add(&mut self, update: StyleUpdate) -> Option<Vec<StyleUpdate>> {
        self.batch.push(update);
        
        if self.should_flush() {
            let batch = std::mem::replace(&mut self.batch, Vec::with_capacity(self.max_batch_size));
            self.last_flush = Instant::now();
            Some(batch)
        } else {
            None
        }
    }

    fn should_flush(&self) -> bool {
        self.batch.len() >= self.max_batch_size || 
        self.last_flush.elapsed() >= self.flush_interval
    }
}
