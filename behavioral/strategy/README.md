# Strategy Pattern in Rust – Implementation Notes

## 1. Concept

The Strategy Pattern encapsulates a family of algorithms, making them interchangeable and independent from the client code that uses them.  
This supports **Open-Closed Principle (OCP)** and **Dependency Inversion Principle (DIP)**, making the code easier to extend and maintain.

---

## 2. Core Implementation Ideas in Rust

Although Rust is not an object-oriented language, it offers `trait` to define behavior contracts, which makes it ideal for strategy pattern implementations.

### ✅ Two Types of Strategies

#### 1. Stateless Strategy

- Contains no internal state;
- Can be reused or cached safely;
- Ideal for pure algorithm logic.

#### 2. Stateful Strategy

- Contains fields (e.g., config, weights);
- Should be instantiated for each use;
- Not safe for sharing across threads unless synchronized.

### ✅ Dynamic Dispatch

Using `Box<dyn Trait>` allows strategies to be chosen at runtime, based on config, user input, or context.

### ✅ Static Dispatch with Enum (Optimization)

For scenarios where all possible strategies are known at compile time, using an `enum` combined with the [enum_dispatch](https://crates.io/crates/enum_dispatch) crate provides a more performant alternative to `Box<dyn Trait>`.

- **Performance**: Avoids dynamic dispatch overhead (vtable lookups) and enables compiler optimizations (inlining).
- **Memory**: Better cache locality compared to heap-allocated trait objects.
- **Ergonomics**: The `#[enum_dispatch]` macro automatically implements the trait for the enum, forwarding calls to the appropriate variant.

Example structure:

```rust
#[enum_dispatch(CmdExc)]
enum OutputFormat {
    Json(JsonObts),
    Csv(CsvObts),
    Yaml(YamlObts),
}
```

---

## 3. When to Use Strategy Pattern

| Scenario                            | Example                           | Recommended |
| ----------------------------------- | --------------------------------- | ----------- |
| Dynamic behavior selection          | Logging, Payment, Discount        | ✅          |
| Structural stability, varying logic | Routing, Compression              | ✅          |
| Too many if-else or match           | File handlers, Message processors | ✅          |

---

## 4. Summary

Rust embraces Strategy Pattern via:

- `trait` as behavior contract
- Structs as concrete strategies
- `Box<dyn Trait>` or generics as injection mechanism

This pattern improves code modularity, testability, and flexibility.
