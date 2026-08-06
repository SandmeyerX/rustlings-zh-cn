// Rust编译器需要知道如何检查提供的引用是否有效，
// 这样它才能将引用在使用前有超出作用域的风险时告知程序员。
// 注意，引用是借用(references are borrows)，并不拥有它们自己的数据。
// 如果它们的所有者超出了作用域会怎样呢?

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //    ^^^^     ^^          ^^          ^^
    if x.len() > y.len() { x } else { y }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
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
