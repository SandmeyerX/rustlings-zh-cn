# 动态数组(Vectors)


动态数组（Vectors）是 Rust 中最常用的数据结构之一。在其他编程语言中，它们往往直接就叫作数组（Arrays），但由于 Rust 是在更底层的层面进行操作的，Rust 中的数组存储在栈上（这意味着它不能增长或收缩，并且其大小必须在编译时确定），而动态数组存储在堆上（这些限制在堆上都不适用）。

动态数组在书中属于较靠后的章节，但我们认为它们非常有用，值得提前介绍。稍后我们还会讨论另一种有用的数据结构——哈希表（hash maps）。

## 对应知识

- [使用 Vector 储存列表](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)（社区中文翻译，原文：[Rust Book: Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)）
- [`iter_mut`（可变迭代器）](https://doc.rust-lang.org/std/primitive.slice.html#method.iter_mut)
- [`map`](https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map)
