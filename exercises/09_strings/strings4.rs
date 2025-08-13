// 对这个函数的调用应该替换为对函数 `string_slice` 或者 `string` 的调用。
fn placeholder() {
    // 这个函数不再需要，因为所有的调用都被替换了
}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: 这里有一堆值 —— 有些是 `String` 类型，有些是 `&str` 类型。
// 你的任务是根据你认为每个值的类型，将 `placeholder(…)` 替换为 `string_slice(…)` 
// 或者 `string(…)`。 
fn main() {
    string_slice("blue");

    string("red".to_string());

    string(String::from("hi"));

    string("rust is fun!".to_owned());

    string("nice weather".into());

    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    string_slice("  hello there ".trim());

    string("Happy Monday!".replace("Mon", "Tues"));

    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
