// TODO: 在不修改此函数的情况下，修复 `main` 函数中的编译器错误。
fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = "green"; // 不要修改此行代码。

    if is_a_color_word(word) {
        println!("那是一个表示颜色的词!");
    } else {
        println!("那似乎不是一个表示颜色的词。");
    }
}
