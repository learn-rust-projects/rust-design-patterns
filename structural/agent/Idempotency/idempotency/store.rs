use std::{collections::HashMap, hash::Hash};

/// Abstract storage trait for idempotency.
/// This allows different storage backends (in-memory, database, etc.)
/// to be plugged in, following the Dependency Inversion Principle.
pub trait IdempotencyStore<K, V> {
    fn get(&self, key: &K) -> Option<&V>;
    fn save(&mut self, key: K, value: V);
}

/// In-memory implementation of IdempotencyStore.
/// Acts as a simple cache.
pub struct MemoryStore<K, V> {
    pub inner: HashMap<K, V>,
}

impl<K: Eq + Hash, V> MemoryStore<K, V> {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
}
impl<K: Eq + Hash, V> Default for MemoryStore<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + Hash, V> IdempotencyStore<K, V> for MemoryStore<K, V> {
    fn get(&self, key: &K) -> Option<&V> {
        self.inner.get(key)
    }
    fn save(&mut self, key: K, value: V) {
        self.inner.insert(key, value);
    }
}
