// 使用位运算中的左移操作来计算 2 的幂。
// `1 << n` 等同于 `2 的 n 次幂`。
fn power_of_2(n: u8) -> u64 {
    1 << n
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn you_can_assert_eq() {
        // TODO: 使用一些值来测试函数 `power_of_2`。
        assert_eq!();
        assert_eq!();
        assert_eq!();
        assert_eq!();
    }
}
