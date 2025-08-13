fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    #[test]
    fn slice_out_of_array() {
        let a = [1, 2, 3, 4, 5];

        // TODO: 从数组 `a` 中获取一个名为 `nice_slice` 的切片，使得测试通过。
        // let nice_slice = ???
        let nice_slice = &a[1..3];
        assert_eq!([2, 3, 4], nice_slice);
    }
}
