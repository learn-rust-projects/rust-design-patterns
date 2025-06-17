# Async Shutdown Hooks in Rust: Callback-based Design with Observer & Template Method Patterns

This project demonstrates how to implement an **asynchronous shutdown hook system** in Rust using **callbacks** as the core mechanism, combined with design pattern principles like the **Observer Pattern** and **Template Method Pattern**.

---

## Key Concepts & Design Patterns

### Callback Mechanism

At the core, each shutdown hook is represented as a **callback**—a function or closure registered by the user to be invoked asynchronously during shutdown. The hooks are stored and triggered dynamically, enabling extensibility and loose coupling.

### Observer Pattern

- **Subject:** `AsyncShutdownHooks` maintains a list of registered hook callbacks (observers).
- **Observers:** Each hook acts as an observer that reacts to the shutdown event.
- When the shutdown signal is received, all registered hooks are notified (called) asynchronously.

### Template Method Pattern (Implicit)

- The `run_hooks()` method defines the overall shutdown process skeleton:
  - Lock and extract all registered hooks.
  - Run all hooks concurrently and wait for completion.
- The specific shutdown tasks are deferred to each hook callback's asynchronous implementation.

---

## How It Works

- Users register asynchronous hooks with `add_hook()`, passing closures that return `Future`s.
- On receiving a termination signal (Ctrl-C), the system triggers all hooks concurrently.
- Each hook runs asynchronously, e.g., closing database connections, flushing logs, etc.
- The main loop continues running until shutdown is triggered.

---

## Why Use This Design?

- **Extensibility:** Users can add any number of async hooks without changing core logic.
- **Decoupling:** The shutdown system doesn't need to know hook internals—only when to run them.
- **Concurrency:** Hooks run in parallel, reducing total shutdown time.
- **Safety:** Proper locking and async mechanisms prevent race conditions.

---

