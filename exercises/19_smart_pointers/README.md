# 智能指针(Smart Pointers)

在 Rust 中，智能指针是一类变量，它们包含内存地址并引用其他数据，但同时还附带额外的元数据和功能。
Rust 的智能指针通常拥有其所指向的数据的所有权，而普通引用只是借用数据。

## 对应知识

- [智能指针](https://kaisery.github.io/trpl-zh-cn/ch15-00-smart-pointers.html)（社区中文翻译，原文：[Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)）
- [使用 Box 指向堆上的数据](https://kaisery.github.io/trpl-zh-cn/ch15-01-box.html)（社区中文翻译，原文：[Using Box to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)）
- [Rc\<T\>，引用计数智能指针](https://kaisery.github.io/trpl-zh-cn/ch15-04-rc.html)（社区中文翻译，原文：[Rc\<T\>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)）
- [共享状态并发](https://kaisery.github.io/trpl-zh-cn/ch16-03-shared-state.html)（社区中文翻译，原文：[Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)）
- [Cow 文档](https://doc.rust-lang.org/std/borrow/enum.Cow.html)
