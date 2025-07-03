# Memento Pattern (Rust Implementation)

> "Without violating encapsulation, capture and externalize an object’s internal state so that the object can be restored to this state later."

---

## Overview

The **Memento Pattern** is a behavioral design pattern that allows capturing and restoring an object’s internal state without exposing its internal structure. It is commonly used to implement **undo/redo**, **rollback**, or **state snapshot** functionality.

The key idea is to **preserve encapsulation** while enabling **state recoverability**.

---

## Structure

```text
┌───────────────┐
│ Originator    │ ← The object that holds the state
└─────┬─────────┘
      │ create/restore
      ▼
┌───────────────┐
│ Memento       │ ← Immutable snapshot of state
└───────────────┘
      ▲
      │ managed by
┌─────┴─────────┐
│ Caretaker     │ ← Stores history (stack, list, etc.)
└───────────────┘
```

- **Originator**: The object whose state needs to be saved/restored. It can create and use Mementos.
- **Memento**: An immutable object that stores the Originator's internal state.
- **Caretaker**: Manages the list of Mementos, typically using a stack or list, and handles undo/redo logic.

---

## Rust Implementation Highlights

- Define a `struct Memento` to store snapshots of state (e.g., a `String`) and implement the `Clone` trait.
- The `Editor` acts as the Originator and exposes `create_memento()` and `restore()` methods.
- The `History` struct acts as the Caretaker, using a `Vec<Memento>` to maintain saved states.
- Encapsulation is preserved: the internal state is never directly accessed or modified from outside.

Key logic:

- The editor modifies its internal state and takes snapshots.
- Undo operations restore the editor to previous states using saved Mementos.
- State safety and encapsulation are strictly maintained.

---

## Typical Use Cases

| Use Case         | Example                          |
|------------------|----------------------------------|
| Text Editor      | Undo / Redo functionality        |
| Game Save System | Save / Load game progress        |
| Transactions     | Rollback in distributed systems  |
| GUI Form         | Restore previous form inputs     |
| Config Manager   | Revert to previous configurations|

---

## Extension Ideas

- Make the Memento generic to support various types of internal state.
- Persist Mementos via JSON or file storage for long-term recovery.
- Support Redo functionality by introducing a dual-stack mechanism.
- Combine with the Command Pattern to support reversible operations.

---
