# Rust Command Pattern Demo

An implementation of the classic behavioral design pattern — Command Pattern — in Rust. This project supports command encapsulation, queued execution, undo operation, and logging. It uses `Rc<RefCell<T>>` for shared state management and `VecDeque` for FIFO command dispatch.

---

## Project Overview

The Command Pattern encapsulates a request as an object, allowing parameterization of actions, queuing, logging, and undo/redo capabilities.

This demo showcases:

- A unified command trait interface
- Concrete command objects
- Shared receiver (`Editor`) via `Rc<RefCell<Editor>>`
- FIFO queue execution via `VecDeque`
- Operation log recording
- Undo support via a command history stack

---

## Project Structure

```text
src/
├── main.rs         // Client entry point
├── editor.rs       // Receiver: editor that performs real actions
├── command.rs      // Command trait and concrete implementations
└── invoker.rs      // Invoker: manages the command queue, logs, and undo
```

## Feature Highlights

| Feature             | Description                                                     |
| ------------------- | --------------------------------------------------------------- |
| Command Abstraction | Encapsulate logic using traits to decouple invoker and receiver |
| Queue Execution     | Use VecDeque to ensure FIFO command execution                   |
| Shared State        | Multiple commands share one Editor via Rc<RefCell<\_>>          |
| Logging             | All operations are recorded for traceability                    |
| Undo Support        | Supports undoing the last executed command with a history stack |
