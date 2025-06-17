# CodeGen Template: A Multi-Language Code Generator using Template Method & Callback in Rust

This project demonstrates how to combine the **Template Method Pattern** with **Callback Interfaces** in Rust to build a clean, extensible code generation system. It allows converting a unified Abstract Syntax Tree (AST) into multiple target languages like JavaScript, C, and Python.

---

## Architecture & Design Principles

### Template Method Pattern

> Define the skeleton of an algorithm in a method, deferring some steps to subclasses or callbacks.

In this project:
- The `CodeGen::generate()` function defines the fixed flow: traverse AST and emit statements.
- The actual language-specific emission (e.g., formatting syntax) is delegated to callbacks (`emit_expr`, `emit_stmt`).

### Callback Mechanism

> Delegate part of the behavior to user-provided callback implementations, enabling decoupling and customization.

- We define a `CodeGenCallback` trait as the callback interface.
- Each language-specific generator (e.g., `JsGen`, `CGen`, `PyGen`) implements this trait.
- The template method invokes the callback dynamically to achieve **pluggable code generation strategies**.

---

## Project Structure

```bash
codegen_template/
├── ast.rs # Defines unified AST types (Expr, Statement)
├── callback.rs # Callback trait: CodeGenCallback
├── template.rs # Template method for orchestrating generation
├── gen_js.rs # JavaScript code generation (callback impl)
├── gen_c.rs # C code generation (callback impl)
├── gen_py.rs # Python code generation (callback impl)
└── main.rs # Demo: Generate code for multiple languages
```