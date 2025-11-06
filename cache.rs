use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub struct ModelCache {
    cache: Arc<DashMap<String, CachedItem>>,
    max_entries: usize,
    ttl: Duration,
}

struct CachedItem {
    data: Vec<f32>,
    timestamp: Instant,
}

impl ModelCache {
    pub fn new(max_entries: usize, ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(DashMap::new()),
            max_entries,
            ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&self, key: &str) -> Option<Vec<f32>> {
        if let Some(item) = self.cache.get(key) {
            if item.timestamp.elapsed() < self.ttl {
                return Some(item.data.clone());
            } else {
                // Remove expired item
                self.cache.remove(key);
            }
        }
        None
    }

    pub fn insert(&self, key: String, data: Vec<f32>) {
        // Evict oldest if cache is full
        if self.cache.len() >= self.max_entries {
            self.evict_oldest();
        }

        self.cache.insert(
            key,
            CachedItem {
                data,
                timestamp: Instant::now(),
            },
        );
    }

    fn evict_oldest(&self) {
        let mut oldest_key: Option<String> = None;
        let mut oldest_time = Instant::now();

        for entry in self.cache.iter() {
            if entry.value().timestamp < oldest_time {
                oldest_time = entry.value().timestamp;
                oldest_key = Some(entry.key().clone());
            }
        }

        if let Some(key) = oldest_key {
            self.cache.remove(&key);
        }
    }

    pub fn clear(&self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.is_empty()
    }
}
