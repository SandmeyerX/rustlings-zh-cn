fn call_me(num: u8) {
    for i in 0..num {
        println!("叮! 调用数字 {}", i + 1);
    }
}

fn main() {
    // TODO: 修复对此函数的调用。
    call_me(3);
}
