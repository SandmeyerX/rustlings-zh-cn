// Clippy 工具是一系列 lint 检查的集合，用于分析代码，从而帮助你发现常见错误并改进 Rust 代码。
//
// 对于这些练习，当存在 Clippy 警告时，代码将无法编译。
// 请查看输出中 Clippy 给出的建议来解决练习。

fn main() {
    // TODO: 修复此行中的 Clippy lint（检查提示）。
    let pi = 3.14;
    let radius: f32 = 5.0;

    let area = pi * radius.powi(2);

    println!("The area of a circle with radius {radius:.2} is {area:.5}");
}
