// TODO: 在 `:` 后添加参数 `num` 缺失的类型。
fn call_me(num:i32) {
    for i in 0..num {
        println!("叮! 调用数字 {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
