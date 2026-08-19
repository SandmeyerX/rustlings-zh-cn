fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完善这个函数，返回较大的数字！
    // 如果两个数字相等，返回它们之中的任何一个都可以。
    // 不要使用：
    // - 另一个函数调用
    // - 额外的变量（仅能使用 `a` 和 `b`）

}

fn main() {
    // 你可以选择性地在此处进行试验。
}

// 暂时不用关注下面的代码 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
