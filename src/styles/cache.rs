use std::sync::Arc;
use parking_lot::RwLock;
use lru::LruCache;

pub struct StyleCache {
    cache: Arc<RwLock<LruCache<String, CompiledStyle>>>,
}

impl StyleCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(RwLock::new(LruCache::new(capacity))),
        }
    }

    pub fn get_or_compute<F>(&self, key: &str, compute: F) -> CompiledStyle 
    where F: FnOnce() -> CompiledStyle {
        if let Some(style) = self.get(key) {
            return style;
        }

        let style = compute();
        self.insert(key.to_string(), style.clone());
        style
    }
}
