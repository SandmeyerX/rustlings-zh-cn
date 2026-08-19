fn main() {
    let number = "T-H-R-E-E";
    println!("拼写出数字: {number}");

    // 使用变量遮蔽（variable shadowing）
    // https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html#遮蔽
    // （社区中文翻译，原文：https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing）
    let number = 3;
    println!("number 加上 2 等于: {}", number + 2);
}
