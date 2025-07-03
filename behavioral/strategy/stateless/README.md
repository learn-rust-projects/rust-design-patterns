# Rust Strategy Pattern in Practice: Payment System (Dynamic Dispatch)

## 1. Business Background

In an e-commerce system, users may pay with different methods: AliPay, WeChat, bank cards, or points.  
Traditional `if-else` logic becomes hard to maintain as payment methods grow.

> We want: **Add a new payment method without modifying existing logic**.

---

## 2. Design Pattern: Strategy Pattern

- **Definition**: Encapsulate each behavior/algorithm in a separate strategy class and use a unified interface to access them.
- **Goal**: Decouple behavior from business logic, support replaceability.
- **Structure**:
  - strategy
    - Trait: Defines the common interface `PaymentStrategy`
    - Structs: Concrete payment implementations (AliPay, WeChat)
  - factory
    - Factory: Responsible for creating and caching strategies
  - order
    - OrderService: Consumes strategy at runtime using dynamic dispatch
    - OrderStaticService: Generic version using static dispatch
  - bin
    - main.rs: Entry point for testing both dynamic and static strategies
---