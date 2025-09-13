//! Business logic module
//!
//! All proxy logic is encapsulated here. The main module doesn't know about the
//! proxy.
//!
//! # Pattern Principles
//! - **Single Responsibility**: Business logic is separate from idempotency
//!   logic.
//! - **Open/Closed Principle**: Add idempotency at composition, not by
//!   modifying original code.

use std::sync::{LazyLock, Mutex};

use crate::{
    idempotency::{proxy::IdempotencyProxy, store::MemoryStore},
    operation::Operation,
};
mod add_one;
use add_one::AddOne;

// Static global in-memory store for idempotency, using Mutex for thread safety.
static STORE: LazyLock<Mutex<MemoryStore<String, i32>>> =
    LazyLock::new(|| Mutex::new(MemoryStore::new()));

/// Runs the add_one operation with idempotency proxy.
/// Demonstrates composing business logic with infrastructure logic via the
/// decorator/proxy pattern.
pub fn run_add_one_with_idempotency(key: Option<String>) {
    let add_one = AddOne(41);
    let mut store = STORE.lock().unwrap();
    // Only the proxy knows about the store and idempotency.
    let mut proxy = IdempotencyProxy::new(add_one, &mut *store);
    let result = proxy.execute(key);
    println!("result: {result}");
}
