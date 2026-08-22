// 测试（Tests）对于确保代码实现预期功能非常重要。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // 在编写单元测试（unit tests）时，
    // 通常会使用通配符从外部（`super`）模块导入所有内容。
    use super::*;

    #[test]
    fn you_can_assert() {
        assert!(is_even(0));
        assert!(!is_even(-1));
        //      ^ 你可以使用取反运算符 `!` 来断言（assert） `false`。
    }
}
