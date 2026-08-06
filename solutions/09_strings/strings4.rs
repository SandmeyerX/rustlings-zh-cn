fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    // 这里，两种处理方式都行得通。
    // `.into()` 方法会将一种类型转换为期望的类型。
    // 如果在期望得到 `String` 类型的地方调用它，它会将 `&str` 转换为 `String` 类型。
    string("nice weather".into());
    // 但是如果在期望得到 `&str` 类型的地方调用它，那么 `&str` 就会保持为 `&str`，因为不需要进行转换(转了个寂寞)。
    // 如果你移除 `#[allow(…)]` 这一行，
    // 那么 Clippy(Rust的一个代码质量检查工具)会告诉你移除下面的 `.into()`，因为这是一个无用的转换。
    #[allow(clippy::useless_conversion)]
    string_slice("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // 警告: 这是字节索引(byte indexing)，而非字符索引(character indexing)。
    // 字符索引可以通过使用 `s.chars().nth(INDEX)` 来完成。
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
