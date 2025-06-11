// Rust 1.70+ thread-safe explicit initialization
use std::sync::{Mutex, OnceLock};

/// Global singleton container with Mutex<Vec<u8>>
pub struct ArraySingleton {
    inner: Mutex<Vec<u8>>,
}

impl ArraySingleton {
    /// Initialize global singleton, can only succeed once
    pub fn try_init() -> Result<(), &'static str> {
        INSTANCE
            .set(Self {
                inner: Mutex::new(vec![]),
            })
            .map_err(|_| "ArraySingleton already initialized")
    }

    /// Get reference to singleton instance, returns None if not initialized
    pub fn get() -> Option<&'static Self> {
        INSTANCE.get()
    }

    /// Safely wrapped push method
    pub fn push(&self, val: u8) {
        let mut vec = self.inner.lock().unwrap();
        vec.push(val);
    }

    /// Safely wrapped length method
    pub fn len(&self) -> usize {
        let vec = self.inner.lock().unwrap();
        vec.len()
    }

    /// Check if the array is empty
    pub fn is_empty(&self) -> bool {
        let vec = self.inner.lock().unwrap();
        vec.is_empty()
    }
}

/// Global static OnceLock controlling singleton lifecycle
static INSTANCE: OnceLock<ArraySingleton> = OnceLock::new();

fn main() {
    // Initialize first (control initialization timing)
    if let Err(e) = ArraySingleton::try_init() {
        println!("Init failed: {e}");
    }

    // Correct usage: ensure initialization is complete before calling
    if let Some(array) = ArraySingleton::get() {
        array.push(42);
        array.push(7);
        println!("Length is: {}", array.len());
        println!("Is empty: {}", array.is_empty());
    } else {
        println!("ArraySingleton not initialized.");
    }

    // Demonstrate failed re-initialization (error handling)
    if let Err(e) = ArraySingleton::try_init() {
        println!("Second init attempt: {e}");
    }
}
