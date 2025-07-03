# Rust Iterator Example

This project demonstrates how to implement a custom iterator in Rust using a simple `Counter` type and a `CounterCollection` container.

## What is an Iterator?

In Rust, an iterator is any type that implements the `Iterator` trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Each call to `next()` returns an element or `None` to indicate the end of the sequence.

## ⚙️ Why Use Iterators?

Iterators provide a **lazy and efficient** way to process sequences, and enable rich, composable operations:

- `.map()`: transform each item;
- `.filter()`: select matching items;
- `.fold()`: reduce into a single value;
- `.collect()`: convert into a collection;
- ... and many more.