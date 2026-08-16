# 错误处理(Error handling)

大多数错误并没有严重到需要让程序完全终止。有些时候，函数执行失败，其失败原因是可以被轻松解析并做出对应处理的。
举个例子：当你尝试打开一个文件，但由于文件不存在而操作失败，这时你或许希望去创建该文件，而不是直接结束进程。

## 对应知识

- [错误处理](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)（社区中文翻译，原文：[Rust Book: Error Handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)）
- [泛型数据类型](https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html)（社区中文翻译，原文：[Rust Book: Generics](https://doc.rust-lang.org/book/ch10-01-syntax.html)）
- [Result](https://rustwiki.org/zh-CN/rust-by-example/error/result.html)（社区中文翻译，原文：[Result](https://doc.rust-lang.org/rust-by-example/error/result.html)）
- [Boxing errors](https://rustwiki.org/zh-CN/rust-by-example/error/multiple_error_types/boxing_errors.html)（社区中文翻译，原文：[Boxing errors](https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html)）
