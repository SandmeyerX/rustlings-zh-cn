fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: 使用元组索引来访问 `numbers` 的第二个元素，
        // 并将其赋值给一个名为 `second` 的变量。
        // let second = ???;

        let second = numbers.1;
        assert_eq!(second, 2, "这不是元组中的第二个数字!");
    }
}
