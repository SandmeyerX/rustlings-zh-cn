fn trim_me(input: &str) -> &str {
    input.trim()
}

fn compose_me(input: &str) -> String {
    // `format!` 宏的语法与 `println!` 相同，不过它会返回一个字符串，
    // 而不是将其打印到终端上。
    // 等同于 `input.to_string() + " world!"`。
    format!("{input} world!")
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
        assert_eq!(trim_me("Hi!"), "Hi!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
