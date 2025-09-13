//! # Global Thread-Safe Counter (LazyLock Singleton)
//!
//! This example demonstrates how to implement a **global singleton counter
//! service** in Rust using [`std::sync::LazyLock`] (stabilized in Rust 1.70).
//!
//! ## What it does
//! - Defines a thread-safe `CounterService`
//! - Lazily initializes a single global instance
//! - Provides methods to increment and read the value
//! - Provides a safe way to access the underlying MutexGuard via a closure
//!
//! ## Why use this pattern
//! - Avoids early initialization (startup cost)
//! - Ensures thread-safe, shared access without unsafe code
//! - Ideal for simple global services: metrics, configs, caches, etc.
//!
//! ## How it works
//! - A `static` global is wrapped in `LazyLock`
//! - `CounterService` uses a `Mutex<u32>` for interior mutability
//! - `with_counter` method exposes a closure-based API for safe, scoped access
//!   to the lock guard
//!
//! ## How to use
//! - Call `GLOBAL_COUNTER.increment()` to increase, or `.get()` to read the
//!   value.
//! - Use `GLOBAL_COUNTER.with_counter(|guard| { ... })` to perform complex
//!   operations holding the lock.
//!
//! ## Design Patterns Applied
//!
//! | Pattern               | Explanation |
//! |-----------------------|-------------|
//! | ✅ Singleton          | `GLOBAL_COUNTER` is the unique global instance, ensured by `LazyLock` for thread-safe lazy initialization. |
//! | ✅ Encapsulation      | `CounterService` encapsulates `Mutex<u32>`, hiding the locking details from users. |
//! | ✅ Facade             | Provides a simple external API (`increment()`, `get()`, `with_counter()`) abstracting concurrency and locking internals. |
//! | ✅ Lazy Initialization| The counter is only created upon first access, saving resources if unused. |

use std::sync::{LazyLock, Mutex};

/// A thread-safe counter service, protected by a Mutex.
///
/// - Internally holds a `u32` counter.
/// - All operations are thread-safe and synchronized.
struct CounterService {
    inner: Mutex<u32>,
}

impl CounterService {
    /// Constructs a new instance of the counter.
    ///
    /// This is called **only once** upon the first access of `GLOBAL_COUNTER`.
    fn new() -> Self {
        println!(">> Initializing CounterService...");
        CounterService {
            inner: Mutex::new(0),
        }
    }

    /// Increments the counter and returns the new value.
    pub fn increment(&self) -> u32 {
        let mut guard = self.inner.lock().unwrap();
        *guard += 1;
        *guard
    }

    /// Returns the current counter value (read-only).
    pub fn get(&self) -> u32 {
        let guard = self.inner.lock().unwrap();
        *guard
    }

    /// Provides access to the underlying `MutexGuard` via a closure.
    ///
    /// This is useful when the caller needs to perform complex or multiple
    /// operations under the same lock scope.
    ///
    /// The closure receives a `MutexGuard<u32>`, ensuring safe, scoped access.
    pub fn with_counter<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut u32) -> R,
    {
        let mut guard = self.inner.lock().unwrap();
        f(&mut guard)
    }
}

/// Global singleton instance of `CounterService`.
///
/// This static is lazily initialized using `LazyLock`
/// and ensures thread-safe, one-time setup.
static GLOBAL_COUNTER: LazyLock<CounterService> = LazyLock::new(CounterService::new);

/// Entry point of the demo application.
fn main() {
    println!("App starting...");

    let val1 = GLOBAL_COUNTER.increment();
    println!("Current value: {val1}");

    let val2 = GLOBAL_COUNTER.increment();
    println!("Current value: {val2}");

    // Use the closure method to safely access the MutexGuard for complex operation
    GLOBAL_COUNTER.with_counter(|counter| {
        *counter += 10;
        println!("After adding 10, value inside lock: {}", *counter);
    });

    println!("Final value: {}", GLOBAL_COUNTER.get());
}
