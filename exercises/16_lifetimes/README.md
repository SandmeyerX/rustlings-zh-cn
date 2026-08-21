# 生命周期(Lifetimes)

生命周期（lifetimes）会告诉编译器如何检查引用是否存活得足够久，以便在任何给定情况下都有效。例如，生命周期会表达这样的约束：“确保参数 `'a'` 与参数 `'b'` 存活得一样久，这样返回值才有效”。

它们仅在借用（即引用）时才是必要的，因为被拷贝的值或是发生移动的值，所有权归属于自身作用域，外部无法对它们进行引用。生命周期意味着可以对函数的调用方代码进行检查，保证传入的实参是有效的。生命周期会对其调用方施加约束。

如果你想进一步了解生命周期标注（lifetime annotations），[lifetimekata](https://tfpk.github.io/lifetimekata/) 项目提供了与 Rustlings 风格类似、但专注于学习编写生命周期标注的练习。

## 对应知识

- [Lifetimes（Rust By Example）](https://rustwiki.org/zh-CN/rust-by-example/scope/lifetime.html)（社区中文翻译，原文：[Lifetimes](https://doc.rust-lang.org/stable/rust-by-example/scope/lifetime.html)）
- [生命周期确保引用有效](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)（社区中文翻译，原文：[Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)）
