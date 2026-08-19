fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // 元组索引语法
        let second = numbers.1;

        assert_eq!(second, 2, "这不是元组中的第二个数字!");
    }
}
