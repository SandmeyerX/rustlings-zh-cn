fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    //      ^^^ 添加 `mut` 关键字，使参数 `vec` 可变。
    vec.push(88);

    vec
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
