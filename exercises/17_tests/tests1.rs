// 测试（Tests）对于确保代码实现预期功能非常重要。

fn is_even(n: i64) -> bool {
    n % 2 == 0
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // TODO: 引入 `is_even`。
    // 你可以使用通配符来引入外部模块中的所有内容。

    #[test]
    fn you_can_assert() {
        // TODO: 使用一些值来测试函数 `is_even`。
        assert!();
        assert!();
    }
}
