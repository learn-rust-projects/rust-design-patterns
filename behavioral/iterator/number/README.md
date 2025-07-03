# Rust Iterator 示例

本项目展示了如何在 Rust 中实现自定义迭代器，使用简单的 `Counter` 类型与 `CounterCollection` 容器。

## 什么是迭代器？

在 Rust 中，迭代器是任何实现了 `Iterator` trait 的类型：

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

每次调用 `next()`，返回一个元素或 `None`（表示结束）。

## ⚙️ 为什么使用迭代器？

迭代器提供了一种**高效、惰性（lazy）**的方式来处理序列数据，并支持丰富的链式操作：

- `.map()`：映射转换；
- `.filter()`：条件过滤；
- `.fold()`：聚合归约；
- `.collect()`：转换为集合类型；
- ……以及更多。