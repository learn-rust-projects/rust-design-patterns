//! Entry point for the demo
//!
//! # Design Pattern Explanation
//!
//! This example demonstrates the Decorator/Proxy Pattern in Rust for adding
//! idempotency to business logic (such as an "add one" operation) without
//! modifying the business logic's code.
//!
//! ## Key OO Principles Demonstrated
//! - **Open/Closed Principle**: Behavior can be extended by composition rather
//!   than modification.
//! - **Single Responsibility Principle**: Business logic and idempotency logic
//!   are clearly separated.
//! - **Composition over Inheritance**: The proxy wraps the operation at
//!   runtime.
//!
//! ## Usage
//! Run the same business operation twice with the same idempotency key
//! and observe that the second call returns the cached result.

mod operation;
mod idempotency {
    pub mod proxy;
    pub mod store;
}
mod business;

fn main() {
    let key = Some("unique-key-123".to_string());
    business::run_add_one_with_idempotency(key.clone());
    business::run_add_one_with_idempotency(key);
}
