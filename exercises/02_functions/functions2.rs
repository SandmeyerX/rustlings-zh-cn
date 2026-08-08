// TODO: 在 `:` 后为参数 `num` 添加缺失的类型。
fn call_me(num:) {
    for i in 0..num {
        println!("叮! 调用数字 {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
