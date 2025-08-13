fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();
    vec.push(88);
    vec
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO: 使动态数组 `vec0` 和 `vec1` 能够被同时访问，以修复测试中的编译器错误。
    #[test]
    fn move_semantics2() {
        let  vec0 = vec![22, 44, 66];

        let vec1 = fill_vec(&vec0);

        assert_eq!(vec0, [22, 44, 66]);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
