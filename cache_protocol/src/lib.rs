use dashmap::DashMap;
use tokio::time::{Duration, Instant};

pub struct CacheEntry {
    pub value: Vec<u8>,
    pub expires_at: Option<Instant>,
}

pub struct SharedCache {
    pub store: DashMap<String, CacheEntry>,
}

pub struct CacheStats {
    pub item_count: usize,
    pub total_bytes: usize,
}

impl SharedCache {
    pub fn new() -> Self {
        Self {
            store: DashMap::new(),
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>, ttl: Option<Duration>) {
        let expires_at = ttl.map(|t| Instant::now() + t);
        self.store.insert(key, CacheEntry { value, expires_at });
    }

    pub fn get(&self, key: &str) -> Option<Vec<u8>> {
        let entry = self.store.get(key)?;

        if let Some(expiry) = entry.expires_at {
            if Instant::now() > expiry {
                drop(entry);
                self.store.remove(key);
                return None;
            }
        }
        Some(entry.value.clone())
    }

    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }

    pub fn stats(&self) -> CacheStats {
        let item_count = self.store.len();
        let total_bytes: usize = self.store.iter().map(|entry| entry.value.len()).sum();

        CacheStats {
            item_count,
            total_bytes,
        }
    }
}
