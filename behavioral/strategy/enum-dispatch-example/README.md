# Rust Strategy Pattern in Practice: Static Dispatch with Enum

## 1. Technical Background

In high-performance scenarios or when all strategy variants are known at compile time, dynamic dispatch (`Box<dyn Trait>`) might introduce unnecessary overhead (vtable lookups, lack of inlining).

> We want: **The flexibility of traits but the performance of static dispatch.**

## 2. Solution: Enum Dispatch

Using the `enum_dispatch` crate, we can wrap implementing structs in an `enum`. Calls to the trait method on the enum are automatically dispatched to the corresponding variant.

- **Performance**: Comparable to manual `match` statements; allows compiler inlining.
- **Memory**: Better cache locality; no heap allocation for the trait object itself.
- **Ergonomics**: Works seamlessly with crates like `strum` for parsing strings into strategies.

## 3. Structure

- **Trait**: `CmdExc` defines the `execute` method.
- **Enum**: `OutputFormat` contains variants `Json`, `Csv`, `Yaml` and implements `CmdExc` via `#[enum_dispatch]`.
- **Strategies**:
  - `JsonObts`
  - `CsvObts`
  - `YamlObts`
- **Main**: Demonstrates parsing input strings directly into the strategy enum and executing it.
