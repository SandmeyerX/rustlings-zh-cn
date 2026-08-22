// 这里展示三种可能的解决方案。

// 使用 `for` 循环和一个可变变量。
fn factorial_for(num: u64) -> u64 {
    let mut result = 1;

    for x in 2..=num {
        result *= x;
    }

    result
}

// 等同于 `factorial_for`，但更简短，且没有显式的 `for` 循环和可变变量。
fn factorial_fold(num: u64) -> u64 {
    // 当 num == 0 时：迭代器 2..=0 为空，
    //                 -> 返回 `fold` 的初始值 1。
    // 当 num == 1 时：迭代器 2..=1 也为空，
    //                 -> 返回初始值 1。
    // 当 num == 2 时：迭代器 2..=2 包含一个元素，
    //                 -> 初始值 1 乘以 2 并返回结果。
    // 当 num == 3 时：迭代器 2..=3 包含两个元素，
    //                 -> 先计算 1 * 2，再将结果 2 乘以第二个元素 3，因此返回结果 6。
    // 依此类推……
    #[allow(clippy::unnecessary_fold)]
    (2..=num).fold(1, |acc, x| acc * x)
}

// 等同于 `factorial_fold`，不过使用了 Clippy 建议的内置方法（built-in method）。
fn factorial_product(num: u64) -> u64 {
    (2..=num).product()
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial_for(0), 1);
        assert_eq!(factorial_fold(0), 1);
        assert_eq!(factorial_product(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial_for(1), 1);
        assert_eq!(factorial_fold(1), 1);
        assert_eq!(factorial_product(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial_for(2), 2);
        assert_eq!(factorial_fold(2), 2);
        assert_eq!(factorial_product(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial_for(4), 24);
        assert_eq!(factorial_fold(4), 24);
        assert_eq!(factorial_product(4), 24);
    }
}
