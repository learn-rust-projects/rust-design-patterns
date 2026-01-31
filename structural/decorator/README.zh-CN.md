# 装饰器模式

**装饰器模式** 允许您在不修改原始代码结构的情况下，动态地向对象添加新的行为。通过将对象"包裹"在一个或多个实现相同接口的装饰器对象中来实现这一点。

## 概念

- **组件（Component）**：定义了可以动态添加职责的对象的接口。
- **具体组件（Concrete Component）**：定义了可以附加额外职责的对象。
- **装饰器（Decorator）**：维护对组件对象的引用，并定义一个符合组件接口的接口。
- **具体装饰器（Concrete Decorator）**：向组件添加职责。

## 示例：I/O 流

本实现通过模拟 I/O 流系统来演示该模式。

### 结构

- **trait `Stream`**：组件接口。
- **结构体 `FileStream`**：表示文件流的具体组件。
- **结构体 `BufferedStream`**：添加缓冲行为的装饰器。
- **结构体 `UppercaseStream`**：将输出转换为大写的装饰器。

### 代码组织

示例位于 `io` 目录中：

- `io/bin.rs`：演示用法的可执行文件。
- `io/lib.rs`：库入口点。
- `io/component/stream.rs`：定义 `Stream` trait 和 `FileStream`。
- `io/stream/buffered_stream.rs`：定义 `BufferedStream`。
- `io/stream/uppercase_stream.rs`：定义 `UppercaseStream`。

## 使用方法

运行示例：

```bash
cd io
cargo run
```

### 输出

示例链式调用装饰器：`UppercaseStream(BufferedStream(FileStream))`。

```text
Decorator 2 [UppercaseStream: buffering data...]
Decorator 1 [BufferedStream: buffering data...]
Final result: HELLO, DECORATOR PATTERN!
```

## 核心优势

### 开放性扩展（OCP）

- **新功能 = 新装饰器**
- 无需修改现有代码即可扩展行为。

### 零侵入

- **不改原类、不改已有调用**
- **对原对象零侵入**
- **调用点零侵入**
  - **调用方式始终一致**：不改调用 = 调用方代码不需要因为"增强行为"而修改调用方式或调用点逻辑。
- **变化点只有：对象构造阶段**

### 组合能力极强

可以自由组合（如"咖啡+奶+糖"），而无需为每种组合（如 `CoffeeWithMilkAndSugar`）创建特定的子类。
