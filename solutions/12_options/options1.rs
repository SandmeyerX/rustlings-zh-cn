// 此函数返回冰箱中剩余的冰淇淋数量。
// 在22:00(24小时制)之前，冰淇淋还剩下5勺。
// 在22:00时，有人会把冰淇淋全部吃完，所以就没有剩余(值为0)。
// 如果 `hour_of_day` 大于23，则返回 `None`。
fn maybe_ice_cream(hour_of_day: u16) -> Option<u16> {
    match hour_of_day {
        0..=21 => Some(5),
        22..=23 => Some(0),
        _ => None,
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_value() {
        // 在测试中使用 `unwrap` 是可以的。
        let ice_creams = maybe_ice_cream(12).unwrap();

        assert_eq!(ice_creams, 5);
    }

    #[test]
    fn check_ice_cream() {
        assert_eq!(maybe_ice_cream(0), Some(5));
        assert_eq!(maybe_ice_cream(9), Some(5));
        assert_eq!(maybe_ice_cream(18), Some(5));
        assert_eq!(maybe_ice_cream(22), Some(0));
        assert_eq!(maybe_ice_cream(23), Some(0));
        assert_eq!(maybe_ice_cream(24), None);
        assert_eq!(maybe_ice_cream(25), None);
    }
}
