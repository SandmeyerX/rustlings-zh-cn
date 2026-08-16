// 此函数返回冰箱中剩余的冰淇淋数量。
// 在 22:00（24 小时制）之前，冰淇淋还剩下 5 勺。
// 在 22:00 时，有人会把冰淇淋全部吃完，所以就没有剩余（值为 0）。
// 如果 `hour_of_day` 大于 23，则返回 `None`。
fn maybe_icecream(hour_of_day: u16) -> Option<u16> {
    // TODO: 完成函数体。
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // TODO: 修复此测试。如何获取 `Option` 中包含的值？
        let icecreams = maybe_icecream(12);

        assert_eq!(icecreams, 5); // 不要修改此行代码。
    }

    #[test]
    fn check_icecream() {
        assert_eq!(maybe_icecream(0), Some(5));
        assert_eq!(maybe_icecream(9), Some(5));
        assert_eq!(maybe_icecream(18), Some(5));
        assert_eq!(maybe_icecream(22), Some(0));
        assert_eq!(maybe_icecream(23), Some(0));
        assert_eq!(maybe_icecream(24), None);
        assert_eq!(maybe_icecream(25), None);
    }
}
