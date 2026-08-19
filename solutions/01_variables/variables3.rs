#![allow(clippy::needless_late_init)]

fn main() {
    // 在 Rust 中，读取未初始化的变量是不被允许的！
    // 因此，我们首先需要给变量赋一个值。
    let x: i32 = 42;

    println!("Number {x}");

    // 可以先声明一个变量，然后稍后再对其进行初始化。
    // 但在初始化之前，它不能被读取使用。
    let y: i32;
    y = 42;
    println!("Number {y}");
}
