# 类型转换(Type conversions)

Rust提供了多种将给定类型的值转换为另一种类型的方法。

类型转换最简单的方法是使用类型转换表达式(type cast expression)，它通过二元运算符(binary operator) `as` 来表示。例如，`println!("{}", 1 + 1.0);` 是无法编译的，因为 `1` 是整数，而 `1.0` 是浮点数。然而，`println!("{}", 1 as f32 + 1.0)` 是可以编译的。练习[`conversions1`](conversions1.rs)大致讲的就是这方面内容。

Rust还提供了一些特征(traits)，在实现(impl)它们之后有助于进行类型转换。这些特征可以在[`convert`](https://doc.rust-lang.org/std/convert/index.html)模块中找到。

这些特征如下:

- 在[`conversions2`](conversions2.rs)中涉及的 `From` 和 `Into`
- 在[`conversions4`](conversions4.rs)中涉及的 `TryFrom` 和 `TryInto`
- 在[`conversions5`](conversions5.rs)中涉及的 `AsRef` 和 `AsMut`

此外，`std::str` 模块提供了一个名为[`FromStr`](https://doc.rust-lang.org/std/str/trait.FromStr.html)的特征，它有助于通过字符串上的 `parse` 方法将字符串转换为目标类型。如果针对给定类型 `Person` 正确实现了该特征，那么 `let p: Person = "Mark,20".parse().unwrap()` 应该既能编译又能运行，且不会出现程序崩溃的情况。

这些应该是***在标准库内***(within the standard library)将数据转换为期望类型的主要方法。 

## 对应知识

这些内容在书中没有直接涉及，但标准库有很好的相关文档。

- [conversions](https://doc.rust-lang.org/std/convert/index.html)
- [`FromStr` trait](https://doc.rust-lang.org/std/str/trait.FromStr.html)
