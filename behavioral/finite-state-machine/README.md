# Rust State Pattern Implementations

The **State Pattern** is a behavioral design pattern that allows an object to change its behavior when its internal state changes. It encapsulates states into separate structs or modules, making transitions clearer, responsibilities more distinct, and logic more decoupled.

This project demonstrates four common implementations of the State Pattern in Rust, each suited for different levels of complexity in state systems.

---

## Overview of Implementation Approaches

| Approach                        | Style             | Characteristics               | Recommended Use Cases                              |
|--------------------------------|-------------------|-------------------------------|-----------------------------------------------------|
| Method 1: Trait + Box\<dyn>     | Object-Oriented   | Highly encapsulated, modular  | Complex behaviors with clear object responsibilities |
| Method 2: Enum + match          | Functional Style  | Simple, high-performance      | Small finite state machines with simple transitions  |
| Method 3: Lookup Table          | Config-Driven     | Centralized, data-friendly    | Systems with many states/events and predictable logic |
| Method 4: Table + Trait Fusion  | Hybrid Architecture | Decoupled data & behavior     | Large-scale state systems with side effects          |

---

## Example Scenario: Mario State Machine

This state machine simulates how Mario's power-up status changes in response to different events.

### States:

- **Small**: Base state
- **Super**: After eating a mushroom
- **Cape**: After getting a cape
- **Fire**: After getting a fire flower

### Events:

- `obtain_mushroom`: Eats a mushroom
- `obtain_cape`: Gets a cape
- `obtain_fire_flower`: Gets a fire flower
- `meet_monster`: Encounters a monster

The behavior of each state is defined by a combination of **state transition** and **score change**.

### State Transition Table:

| Current State | E1: Mushroom     | E2: Cape        | E3: Fire Flower | E4: Monster     |
|---------------|------------------|-----------------|------------------|------------------|
| Small         | Super / +100     | Cape / +200     | Fire / +300      | No Change        |
| Super         | Super / 0        | Cape / +200     | Fire / +300      | Small / -100     |
| Cape          | Cape / 0         | Cape / 0        | Cape / 0         | Small / -200     |
| Fire          | Fire / 0         | Fire / 0        | Fire / 0         | Small / -300     |

---

## Method 1: Trait-based State Machine

This object-oriented implementation defines one struct per state, each implementing a shared trait. Each state struct contains its own transition logic and interacts with a shared state machine.

**Advantages:**

- Clear separation of responsibilities
- Easy to extend and maintain
- Supports state-specific behaviors

**Disadvantages:**

- Runtime dynamic dispatch has some overhead
- More verbose and complex structure

**Best For:**

- Simple transition logic with rich state behavior

---

## Method 2: Enum + match-based State Machine

This approach uses an enum to represent all states and handles event transitions with `match` statements.

**Advantages:**

- Simple, concise, high performance
- Ideal for rapid prototyping

**Disadvantages:**

- Hard to scale or extend
- Coupled state and behavior logic (violates open-closed principle)

**Best For:**

- Small state sets
- Low-change, clearly defined systems

---

## Method 3: Lookup Table State Machine

This approach encodes state transitions and score changes into two 2D arrays: `transition_table` and `action_table`. Transitions are determined by table lookups based on the current state and event.

**Advantages:**

- Centralized, clean transition logic
- Compatible with config files (YAML / JSON / DSL)
- Easy to maintain and expand

**Disadvantages:**

- Cannot represent side effects (e.g., logging, UI)
- Logic limited by static tables

**Best For:**

- Large number of states/events
- Simple, data-driven systems
- Configurable automata

---

## Method 4: Lookup Table + Trait Hybrid

This hybrid model combines lookup tables with the trait-based pattern:

- `TRANSITIONS` and `SCORES` manage state changes and scores
- A `MarioState` trait encapsulates state-specific side effects
- Dynamic dispatch invokes side effects before transitions

**Event Flow:**

1. Current state executes its side effect (e.g., logging)
2. State machine performs lookup to get next state and score
3. Updates current state and score accordingly

**Advantages:**

- Clean separation of data and behavior
- Composable and testable state logic
- Enables dynamic table switching or hot config reloads

**Disadvantages:**

- Slightly more complex to implement
- Needs additional design to inject context for behaviors

**Best For:**

- Systems with many states/events and moderate complexity
- Combines configurability and behavioral customization
- Game logic, UI animation systems, transaction modeling, etc.

---

## Comparison & Recommendation

| Implementation Style         | Encapsulation | Extensibility | Performance | Complexity | Recommended Scenarios                                             |
|-----------------------------|---------------|---------------|-------------|------------|--------------------------------------------------------------------|
| Trait + Box                 | High          | High          | Medium      | Medium     | Behavior-heavy logic with clean separation of responsibilities     |
| Enum + match                | Low           | Low           | High        | Low        | Simple systems with limited states                                |
| Lookup Table                | Medium        | Medium        | High        | Low        | Event/state-heavy, config-driven systems                          |
| Table + Trait Hybrid        | High          | High          | High        | Medium     | Scalable systems needing side effects + data-driven transitions    |

---
