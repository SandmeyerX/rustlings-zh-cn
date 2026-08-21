// 小明正在买苹果。一个苹果的价格按如下方式计算：
// - 一个苹果售价 2r（Rust 国的货币 —— rua元，简称 r）。
// - 不过，如果购买数量超过 40 个，那么整个订单中每个苹果的价格将降至仅 1r！

fn calculate_price_of_apples(n_apples: u64) -> u64 {
    if n_apples > 40 {
        n_apples
    } else {
        2 * n_apples
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

// 不要修改下面的测试！
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);
        assert_eq!(calculate_price_of_apples(40), 80);
        assert_eq!(calculate_price_of_apples(41), 41);
        assert_eq!(calculate_price_of_apples(65), 65);
    }
}
