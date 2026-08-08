#![allow(clippy::ptr_arg)]

// 借用（borrow）而不是获取所有权（ownership）。
// 这里建议使用 `&str` 而不是 `&String`。但目前这样就足够了，因为我们还没处理到字符串的内容。
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// 获取所有权，而不是借用。
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let data = "Rust真不错!".to_string();

    get_char(&data);

    string_uppercase(data);
}
