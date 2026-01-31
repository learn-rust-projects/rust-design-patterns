# AutoStrategy - Automatic Strategy Pattern in Rust

This crate demonstrates an advanced implementation of the Strategy Pattern in Rust with automatic registration of strategies using macros.

## Features

- **Trait-based Strategy Interface**: Defines a common interface for all payment strategies
- **Automatic Registration**: Strategies are automatically registered at runtime using macros and the `ctor` crate
- **Thread Safety**: Uses `LazyLock` and `Mutex` to ensure thread-safe access to the strategy registry
- **Type Safety**: Leverages Rust's type system to ensure safe strategy implementation and usage

## Implementation Details

The implementation consists of several key components:

1. **PaymentStrategy Trait**: Defines the interface that all payment strategies must implement
2. **Global Registry**: Uses a thread-safe `HashMap` to store strategy constructors
3. **define_strategy! Macro**: Simplifies the creation and registration of new strategies
4. **Automatic Registration**: Uses the `ctor` crate to register strategies at program startup

## Usage

To define a new payment strategy, simply use the `define_strategy!` macro:

```rust
define_strategy!(
    CreditCardPay,
    "credit_card",
    |_, amount| format!("Credit Card Payment: {} yuan", amount)
);
```

Strategies are automatically registered and can be accessed through the global registry.

## Dependencies

- `lazy_static` or Rust's standard `LazyLock` (for Rust 1.70+)
- `paste` crate for macro string manipulation
- `ctor` crate for automatic registration at program startup

## Running the Example

```bash
cargo run
```

This will execute the main function which demonstrates the automatic registration and usage of payment strategies.

## License

This project is licensed under the MIT License.
