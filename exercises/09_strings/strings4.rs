// 对这个函数的调用应该替换为对函数 `string_slice` 或者 `string` 的调用。
fn placeholder() {}

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
    placeholder("blue");

    placeholder("red".to_string());

    placeholder(String::from("hi"));

    placeholder("rust is fun!".to_owned());

    placeholder("nice weather".into());

    placeholder(format!("Interpolation {}", "Station"));
    
    // 警告: 这是字节索引(byte indexing)，而非字符索引(character indexing)。
    // 字符索引可以通过使用 `s.chars().nth(INDEX)` 来完成。
    placeholder(&String::from("abc")[0..1]);

    placeholder("  hello there ".trim());

    placeholder("Happy Monday!".replace("Mon", "Tues"));

    placeholder("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
