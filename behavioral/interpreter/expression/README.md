# The Interpreter Pattern Demo

The Interpreter Pattern is a behavioral design pattern used to define a grammar for a language and build an interpreter to evaluate expressions written in that language. The core idea is to abstract the grammar rules into a class structure (AST - Abstract Syntax Tree), compose these structures, and define a unified interface to interpret or execute the language logic programmatically.

This pattern is especially suitable for scenarios such as:

- Implementing simple scripting languages or domain-specific languages (DSLs)
- Modeling and evaluating rule-based expressions (e.g., math formulas, logic expressions, filters)
- Expression evaluators in state machines, formula calculators, or rule engines

---

### Pattern Structure

| Role                          | Responsibility                                                      |
|-------------------------------|----------------------------------------------------------------------|
| Abstract Expression           | Defines the interpret operation interface (e.g., `interpret()`)     |
| Terminal Expression           | Represents atomic elements like numbers or constants                |
| Non-Terminal Expression       | Represents composite expressions such as addition or subtraction    |
| Client                        | Builds the expression tree and invokes the interpret logic          |

---

This project demonstrates the pattern using addition and subtraction expressions. By composing multiple expression objects, an abstract syntax tree is built, and the expression is evaluated recursively by calling the `interpret()` method.

For example, the expression:

`5 + (3 - 2)`

Can be represented as the following syntax tree:

```text
    Add
   /   \
 5     Subtract
       /     \
     3        2
```

The evaluation proceeds recursively from the bottom of the tree upwards, producing the final result 6.