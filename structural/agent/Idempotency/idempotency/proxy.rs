use super::store::IdempotencyStore;
use crate::operation::Operation;

/// IdempotencyProxy is a Decorator/Proxy for Operation.
/// It adds idempotency logic transparently to any operation without changing its code.
/// - Decorator/Proxy Principle: Enhances an object by wrapping it with new behavior.
/// - Open/Closed Principle: Add features by composition, not modification.
pub struct IdempotencyProxy<'a, O, T, K = String>
where
    O: Operation<T>,
    K: Clone,
{
    operation: O,
    store: &'a mut dyn IdempotencyStore<K, T>,
    _phantom: std::marker::PhantomData<(T, K)>,
}

impl<'a, O, T, K> IdempotencyProxy<'a, O, T, K>
where
    O: Operation<T>,
    K: Clone,
{
    /// Wrap an operation with idempotency logic.
    pub fn new(operation: O, store: &'a mut dyn IdempotencyStore<K, T>) -> Self {
        Self {
            operation,
            store,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<'a, O, T, K> Operation<T> for IdempotencyProxy<'a, O, T, K>
where
    O: Operation<T>,
    K: Clone + From<String> + Into<String>,
    T: Clone,
{
    /// If the key exists in the store, return the cached result;
    /// else, execute the operation and cache the result.
    fn execute(&mut self, idempotency_key: Option<String>) -> T {
        if let Some(key) = idempotency_key {
            let key: K = key.into();
            if let Some(result) = self.store.get(&key) {
                println!("Cache hit: returning store result");
                return result.clone();
            }
            println!("Cache miss: executing actual operation");
            let result = self.operation.execute(Some(key.clone().into()));
            self.store.save(key, result.clone());
            result
        } else {
            self.operation.execute(None)
        }
    }
}
