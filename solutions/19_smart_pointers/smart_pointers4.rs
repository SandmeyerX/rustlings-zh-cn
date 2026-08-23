// 本练习将探究 `Cow`（Clone-On-Write, 写时克隆）智能指针。
// 它封装并提供了对借用数据的不可变访问，
// 同时，当需要修改数据或获取其所有权时，它会惰性地克隆数据。
// 这种类型旨在通过 `Borrow` 特征来处理各种通用的借用数据。

use std::borrow::Cow;

fn abs_all(input: &mut Cow<[i32]>) {
    for ind in 0..input.len() {
        let value = input[ind];
        if value < 0 {
            // 如果尚未拥有所有权，则克隆到一个动态数组中。
            input.to_mut()[ind] = -value;
        }
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reference_mutation() {
        // 发生克隆，是因为 `input` 需要被修改（mutated）。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
    }

    #[test]
    fn reference_no_mutation() {
        // 不发生克隆，是因为 `input` 不需要被修改。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(&vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Borrowed(_)));
        //                      ^^^^^^^^^^^^^^^^
    }

    #[test]
    fn owned_no_mutation() {
        // 我们也可以不使用 `&`，直接传递 `vec`，
        // 这样 `Cow` 就能直接拥有它的所有权。
        // 在这种情况下，不会发生修改操作（因为所有数字已经是绝对值了），因此也不会发送克隆。
        // 但由于它从未被借用或修改，结果依然是拥有所有权状态。
        let vec = vec![0, 1, 2];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }

    #[test]
    fn owned_mutation() {
        // 当然，如果确实发生了修改操作（并非所有数字都是绝对值），情况也是如此。
        // 在这种情况下，`abs_all` 函数中对 `to_mut()` 的调用会返回一个指向与之前相同数据的引用。
        let vec = vec![-1, 0, 1];
        let mut input = Cow::from(vec);
        abs_all(&mut input);
        assert!(matches!(input, Cow::Owned(_)));
        //                      ^^^^^^^^^^^^^
    }
}
