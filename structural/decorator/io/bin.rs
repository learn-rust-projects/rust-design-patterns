//! Decorator Pattern Demo in Rust
//!
//! # Design Pattern Explanation
//!
//! The **Decorator Pattern** allows you to dynamically add new behavior to
//! objects without altering the structure of the original code. You achieve
//! this by "wrapping" an object in one or more decorator objects that implement
//! the same interface.
//!
//! ## Principles Illustrated
//!
//! - **Open/Closed Principle:** Add functionality by composition, not
//!   modification.
//! - **Single Responsibility Principle:** Each decorator encapsulates a single
//!   enhancement.
//! - **Favor composition over inheritance:** Decorators wrap components,
//!   avoiding deep inheritance trees.
//!
//! ## Structure
//!
//! - `Stream` trait: the base abstraction (Component)
//! - `FileStream`: the base implementation (ConcreteComponent)
//! - `BufferedStream`, `UppercaseStream`: decorators that add responsibilities
//!
//! ## Usage Example
//!
//! Demonstrates how decorators can be flexibly stacked.

use decorator::{
    component::stream::{FileStream, Stream},
    stream::{buffered_stream::BufferedStream, uppercase_stream::UppercaseStream},
};

fn main() {
    // Base component: a file stream
    let file_stream = FileStream;

    // Decorator 1: adds buffering
    let buffered = BufferedStream::new(file_stream);

    // Decorator 2: adds uppercase transformation
    let mut uppercased = UppercaseStream::new(buffered);

    // Read from the decorated stream
    let result = uppercased.read();

    // Output: "BUFFEREDSTREAM: BUFFERING DATA...\nHELLO, DECORATOR PATTERN!"
    println!("Final result: {result}");
}
