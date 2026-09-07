# 异步编程

异步编程是一种将任务委托给运行时、由运行时并发执行的模型。
它在需要执行大量独立 IO 操作的应用场景下特别高效，例如 Web 服务器。

Rust 语言本身提供了实现异步编程所需的原语。
但 Rust 标准库中并未内置运行时。
在本节练习中，我们将使用名为 `tokio` 的主流运行时。

## 对应知识

- [异步编程基础](https://kaisery.github.io/trpl-zh-cn/ch17-00-async-await.html)（社区中文翻译，原文：[Fundamentals of Asynchronous Programming](https://doc.rust-lang.org/book/ch17-00-async-await.html)）
- [Tokio 文档](https://docs.rs/tokio/latest/tokio/)
