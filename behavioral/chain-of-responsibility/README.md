# Chain of Responsibility Pattern in Rust

**Chain of Responsibility** is a behavioral design pattern that allows a request to be passed along a chain of handlers until one of them handles it or the chain ends.

This example demonstrates two common implementations in Rust:

- **Array-based (Vec)**: Best suited for static chains or when the number of handlers is fixed
- **Linked-list-based (Box)**: Offers greater flexibility for building dynamic chains

## Pattern Structure

```text
Handler (trait interface)
├── HandlerA (concrete handler)
├── HandlerB (concrete handler)
└── ...

HandlerChain (handler chain container)
└── Sequentially invokes handlers until one succeeds or the chain ends
```

## Variant

In addition to the standard implementation, the following variant is also provided:

### All handlers execute (ignore whether handled successfully)

In some scenarios (e.g., logging, monitoring, data pipelines), we want every handler to execute regardless of whether the request is considered "handled."  
Implemented via the `handle_without_check` method.

## Recommendations

- If the handler chain is fixed, prefer the **Vec-based** implementation for simplicity and maintainability.
- If dynamic insertion/removal or complex structures are required, prefer the **Box-based linked list**.
- If all handlers should be executed regardless of success, use the `handle_without_check` method.
