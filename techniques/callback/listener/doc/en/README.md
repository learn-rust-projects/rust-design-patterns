# Button Click Event Example in Rust Using Observer Pattern, Template Method Pattern, and Callback Mechanism

This example demonstrates how to implement an event-driven system for a UI control (Button) in Rust by combining the **Observer Pattern**, **Template Method Pattern**, and **Callback Mechanism** design principles.

---

## Design Patterns & Concepts

### Observer Pattern

- **Purpose:** Define a one-to-many dependency so that when one object changes state, all its dependents are automatically notified and updated.  
- **In this example:**  
  - `Button` acts as the **Subject** (publisher), maintaining a list of listeners (observers).  
  - Each listener implements the `ClickListener` trait (observer interface).  
  - When the button is clicked, it notifies all registered listeners by calling their `on_click()` methods.  

### Template Method Pattern (Implicit)

- **Purpose:** Define the skeleton of an algorithm in a method, deferring some steps to subclasses or callback implementations.  
- **In this example:**  
  - The `Button::click()` method defines the fixed flow of a button click event: logging the click, then notifying all listeners.  
  - The specific response to the click event is deferred to each listener’s implementation of `on_click()`, allowing flexible customization.  

### Callback Mechanism

- **Purpose:** Delegate part of the behavior to user-defined callback interfaces to decouple core logic from specific business responses.  
- **In this example:**  
  - The `ClickListener` trait defines the callback interface, enabling users to implement custom event handling logic.  
  - The `Button` holds multiple callback objects implementing this interface and invokes them uniformly when the event occurs, achieving flexible multi-cast event dispatching.  

---

This combination of design patterns ensures **low coupling**, **high extensibility**, and **separation of concerns** in event-driven programming, making it well-suited for various event-based business scenarios.
