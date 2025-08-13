#![allow(clippy::ptr_arg)]

// TODO: 仅通过添加或删除引用(`&` 字符)来修复编译器错误，
// 除此之外不要做任何更改。

// 不应该取得所有权
fn get_char(data: & String) -> char {
    data.chars().last().unwrap()
}

// 应该取得所有权
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{data}");
}

fn main() {
    let  data = "Rust真不错!".to_string();

    get_char(&data);

    string_uppercase(data);
}
