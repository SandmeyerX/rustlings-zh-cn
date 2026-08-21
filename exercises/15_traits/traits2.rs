trait AppendBar {
    fn append_bar(self) -> Self;
}

// TODO: 为字符串动态数组实现 `AppendBar` 特征。
// `append_bar` 方法应该将字符串 "Bar" 推入（push）到该动态数组中。

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut foo = vec![String::from("Foo")].append_bar();
        assert_eq!(foo.pop().unwrap(), "Bar");
        assert_eq!(foo.pop().unwrap(), "Foo");
    }
}
