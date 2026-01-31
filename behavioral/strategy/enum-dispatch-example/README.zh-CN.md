# Rust 策略模式实战：基于 Enum 的静态分发

## 一、技术背景

在高性能场景下，或者当所有策略变体在编译期已知时，动态分发（`Box<dyn Trait>`）可能会带来不必要的开销（vtable 查找、无法内联）。

> 我们希望：**既拥有 Trait 的灵活性，又具备静态分发的性能。**

## 二、解决方案：Enum Dispatch

使用 `enum_dispatch` crate，我们可以将实现类包装在一个 `enum` 中。对 enum 调用 Trait 方法时，会自动分发到对应的变体上。

- **性能**：媲美手写 `match` 语句；允许编译器进行内联优化。
- **内存**：更好的缓存局部性；不需要为 trait object 进行堆分配。
- **易用性**：可以结合 `strum` 等 crate 轻松实现从字符串到策略的解析。

## 三、代码结构

- **Trait**：`CmdExc` 定义了 `execute` 接口。
- **Enum**：`OutputFormat` 包含 `Json`、`Csv`、`Yaml` 变体，并通过 `#[enum_dispatch]` 实现 `CmdExc`。
- **具体策略**：
  - `JsonObts`
  - `CsvObts`
  - `YamlObts`
- **入口**：演示将输入字符串直接解析为策略 Enum 并执行。
