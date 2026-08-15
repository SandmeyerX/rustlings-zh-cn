fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}

fn main() {
    let word = String::from("green");

    if is_a_color_word(&word) {
        //             ^ 加这个 `&` 是为了得到 `&String` 类型，
        //               编译器会自动将其强制转换为 `&str` 类型。
        println!("那是一个表示颜色的词!");
    } else {
        println!("那不是一个表示颜色的词。");
    }
}
