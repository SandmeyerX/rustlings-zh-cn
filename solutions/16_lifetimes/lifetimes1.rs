// Rust 编译器需要知道如何检查传入的引用是否有效，
// 这样它就能告知程序员：某个引用是否存在在被使用前就离开作用域的风险。
// 记住，引用就是借用（references are borrows），并不拥有自身的数据。
// 如果它们的所有者离开了作用域，会怎样呢？

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //    ^^^^     ^^          ^^          ^^
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abcd", "123"), "abcd");
        assert_eq!(longest("abc", "1234"), "1234");
    }
}
