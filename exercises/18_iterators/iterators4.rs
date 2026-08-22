fn factorial(num: u64) -> u64 {
    // TODO: 完成此函数，使其返回 `num` 的阶乘，
    // `num` 的阶乘定义为 `1 * 2 * 3 * … * num`
    //   - https://zh.wikipedia.org/wiki/階乘
    //   - （大陆可访问百度百科）https://baike.baidu.com/item/阶乘/4437932
    //
    // 请勿使用：
    //   - 提前返回（显式使用 `return` 关键字）
    // 尽量不使用：
    //   - 命令式风格的循环（`for`/`while`）
    //   - 额外的变量
    // 进阶挑战，不使用：
    //   - 递归
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(factorial(1), 1);
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(factorial(4), 24);
    }
}

