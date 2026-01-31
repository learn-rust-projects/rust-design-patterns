# Decorator Pattern

The **Decorator Pattern** allows you to dynamically add new behavior to objects without altering the structure of the original code. You achieve this by "wrapping" an object in one or more decorator objects that implement the same interface.

## Concept

- **Component**: Defines the interface for objects that can have responsibilities added to them dynamically.
- **Concrete Component**: Defines an object to which additional responsibilities can be attached.
- **Decorator**: Maintains a reference to a Component object and defines an interface that conforms to Component's interface.
- **Concrete Decorator**: Adds responsibilities to the component.

## Example: I/O Streams

This implementation demonstrates the pattern using a simulated I/O stream system.

### Structure

- **Trait `Stream`**: The component interface.
- **Struct `FileStream`**: The concrete component representing a file stream.
- **Struct `BufferedStream`**: A decorator that adds buffering behavior.
- **Struct `UppercaseStream`**: A decorator that transforms the output to uppercase.

### Code Organization

The example is located in the `io` directory:

- `io/bin.rs`: The main executable demonstrating the usage.
- `io/lib.rs`: The library entry point.
- `io/component/stream.rs`: Defines `Stream` trait and `FileStream`.
- `io/stream/buffered_stream.rs`: Defines `BufferedStream`.
- `io/stream/uppercase_stream.rs`: Defines `UppercaseStream`.

## Usage

To run the example:

```bash
cd io
cargo run
```

### Output

The example chains decorators: `UppercaseStream(BufferedStream(FileStream))`.

```text
Decorator 2 [UppercaseStream: buffering data...]
Decorator 1 [BufferedStream: buffering data...]
Final result: HELLO, DECORATOR PATTERN!
```

## Key Advantages

### Open/Closed Principle

- **New functionality = New decorator**
- Extend behavior without modifying existing code.

### Zero Intrusion

- **No changes to original class or existing calls**
- **Zero intrusion on the original object**
- **Zero intrusion at the call site**
  - **Calling convention remains consistent**: No need to change how the object is used.
  - **The only change point is in the object construction phase**.

### Powerful Composability

Decorators can be freely combined (like "coffee + milk + sugar") without creating specific subclasses for each combination (e.g., `CoffeeWithMilkAndSugar`).
