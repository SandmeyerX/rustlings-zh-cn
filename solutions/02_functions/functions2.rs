// 函数参数必须标注类型。
// 添加了类型注解 `u64`。
fn call_me(num: u64) {
    for i in 0..num {
        println!("叮! 调用数字 {}", i + 1);
    }
}

fn main() {
    call_me(3);
}
