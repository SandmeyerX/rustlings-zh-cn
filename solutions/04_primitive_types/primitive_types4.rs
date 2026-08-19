fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];
        //       0  1  2  3  4  <- 索引（index，复数 indices）
        //          -------
        //             |
        //             +--- 切片（slice，这里是 &[2, 3, 4]）

        // 请注意，上限索引（upper index）4 是不包含在内的。
        let nice_slice = &a[1..4];
        assert_eq!([2, 3, 4], nice_slice);

        // 使用 `..=` 语法（带有 `=` 符号），上限索引可以被包含在内。
        let nice_slice = &a[1..=3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
