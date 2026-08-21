# 特征(Traits)

特征（trait）是一组方法的集合。

数据类型可以实现特征。要实现特征，就需要为该数据类型定义组成特征的各个方法。例如，`String` 数据类型实现了 `From<&str>` 特征，这使得用户可以编写 `String::from("hello")` 这样的代码。

从这个角度来看，特征与 Java 中的接口（interfaces）以及 C++ 中的抽象类（abstract classes）有几分相似。

Rust 中其他一些常用特征包括：

- `Clone`（提供 `clone` 方法）
- `Display`（允许通过 `{}` 进行格式化显示）
- `Debug`（允许通过 `{:?}` 进行格式化显示）

由于特征描述了不同数据类型之间共有的行为，因此在编写泛型时它们非常有用。

## 对应知识

- [Trait：定义共同行为](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)（社区中文翻译，原文：[Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)）
