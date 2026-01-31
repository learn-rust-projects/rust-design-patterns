# AutoStrategy - Rust中的自动策略模式实现

本项目展示了在Rust中使用宏实现策略模式的高级方式，支持策略的自动注册。

## 特性

- **基于Trait的策略接口**：为所有支付策略定义了通用接口
- **自动注册**：使用宏和`ctor`库在运行时自动注册策略
- **线程安全**：使用`LazyLock`和`Mutex`确保对策略注册表的线程安全访问
- **类型安全**：利用Rust的类型系统确保策略实现和使用的安全性

## 实现细节

该实现包含几个关键组件：

1. **PaymentStrategy Trait**：定义所有支付策略必须实现的接口
2. **全局注册表**：使用线程安全的`HashMap`存储策略构造函数
3. **define_strategy! 宏**：简化新策略的创建和注册过程
4. **自动注册**：使用`ctor`库在程序启动时自动注册策略

## 使用方法

要定义新的支付策略，只需使用`define_strategy!`宏：

```rust
define_strategy!(
    CreditCardPay,
    "credit_card",
    |_, amount| format!("信用卡支付: {} 元", amount)
);
```

策略会自动注册，并可以通过全局注册表访问。

## 依赖

- `lazy_static`或Rust标准库的`LazyLock`（适用于Rust 1.70+）
- `paste`库，用于宏字符串操作
- `ctor`库，用于在程序启动时自动注册

## 运行示例

```bash
cargo run
```

这将执行main函数，展示支付策略的自动注册和使用方法。

## 许可证

本项目采用MIT许可证。
