# Rust Strategy Pattern in Action: Log Output System

## 🎯 1. Design Pattern Overview: Strategy Pattern

**The Strategy Pattern** defines a family of interchangeable algorithms, encapsulates each one, and makes them interchangeable.  
This pattern allows the algorithm to vary independently from clients that use it.

- **Purpose**: Eliminate lengthy `if-else` or `match` blocks by making behaviors pluggable.
- **Key Roles**:
  - Strategy Interface (in this case: `fn(&str)` function pointers)
  - Concrete Strategies (console, file, network, etc.)
  - Strategy Factory (returns strategy by enum type)
  - Context (here: `Logger`)

---

## 📦 2. Business Scenario: Pluggable Logging Output System

A system needs to output logs to various backends depending on the environment:

- Console output (e.g. in dev mode)
- File logging (for local persistence)
- Network output (for centralized logging servers like ELK)
- Temporary Closure Strategy

You want the ability to dynamically select or inject a logging strategy at runtime.

---