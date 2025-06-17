# 代码生成模板：基于模板方法与回调机制的多语言代码生成器（Rust 实现）

本项目演示了如何在 Rust 中结合 **模板方法模式** 和 **回调接口**，构建一个简洁且可扩展的代码生成系统。它能够将统一的抽象语法树（AST）转换成多种目标语言，如 JavaScript、C 和 Python。

---

## 架构与设计原则

### 模板方法模式

> 在一个方法中定义算法的骨架，将部分步骤延迟到子类或回调中实现。

在本项目中：
- `CodeGen::generate()` 函数定义了固定流程：遍历 AST 并生成语句。
- 具体的语言相关代码生成（如语法格式化）则由回调方法 (`emit_expr`, `emit_stmt`) 完成。

### 回调机制

> 将部分行为委托给用户提供的回调实现，从而实现解耦和定制化。

- 定义了 `CodeGenCallback` trait 作为回调接口。
- 各语言特定的生成器（如 `JsGen`、`CGen`、`PyGen`）实现该 trait。
- 模板方法动态调用回调，实现**可插拔的代码生成策略**。

---

## 项目结构

```bash
codegen_template/
├── ast.rs           # 定义统一的 AST 类型（表达式 Expr，语句 Statement）
├── callback.rs      # 回调接口定义：CodeGenCallback trait
├── template.rs      # 模板方法，用于调度代码生成流程
├── gen_js.rs        # JavaScript 代码生成（回调实现）
├── gen_c.rs         # C 语言代码生成（回调实现）
├── gen_py.rs        # Python 代码生成（回调实现）
└── main.rs          # 演示：生成多语言代码示例
```