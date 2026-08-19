# Options

`Option` 类型表示一个可选值：每个 `Option` 要么是 `Some` 并包含一个值，要么是 `None` 且不包含值。

在 Rust 代码中，`Option` 类型非常常见，因为它们有许多用途：
- 初始值（Initial values）
- 无法对全部输入定义域给出有效输出的函数返回值（偏函数，Partial functions）
- 用于简单错误上报的返回值：出错时返回 `None`
- 结构体的可选字段
- 可以被借出或“拿走（所有权）”的结构体字段
- 函数的可选参数
- 可空指针（Nullable pointers）
- 在复杂场景下完成数据置换

## 对应知识

- [枚举定义中的泛型](https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html#枚举定义中的泛型)（社区中文翻译，原文：[Rust Book: In Enum Definitions](https://doc.rust-lang.org/book/ch10-01-syntax.html#in-enum-definitions)）
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://rustwiki.org/zh-CN/rust-by-example/flow_control/if_let.html)（社区中文翻译，原文：[if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)）
- [while let](https://rustwiki.org/zh-CN/rust-by-example/flow_control/while_let.html)（社区中文翻译，原文：[while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)）
