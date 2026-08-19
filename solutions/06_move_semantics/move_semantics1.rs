fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    //  ^^^ 添加 `mut` 关键字，使 `vec` 可变。

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
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        // `vec0` 无法再被访问，因为它已经被移动（move）到 `fill_vec` 中了。
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}
