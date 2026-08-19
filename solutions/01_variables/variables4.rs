fn main() {
    // 在 Rust 中，变量默认是不可变的。
    // 在 `let` 后添加 `mut` 关键字可以声明变量是可变的。
    let mut x = 3;
    println!("Number {x}");

    x = 5;
    println!("Number {x}");
}
