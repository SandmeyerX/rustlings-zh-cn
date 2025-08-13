// TODO: 在不改变函数签名的前提下，修复编译器错误。
fn current_favorite_color() -> String {
    "blue".to_string()
}

fn main() {
    let answer = current_favorite_color();
    println!("我目前喜欢的颜色是 {answer}");
}
