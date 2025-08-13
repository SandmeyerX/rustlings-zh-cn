// 需要定义一个用哈希表来表示的水果篮。
// 键表示水果的名称，值表示该种水果在篮子中的数量。
// 你必须在篮子中放入至少3种不同类型的水果(例如苹果、香蕉、芒果)，
// 并且所有水果的总数至少为5。

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    // TODO: 声明哈希表。
    // let mut basket =

    let mut basket = HashMap::new();
    // 已经给你准备好了两根香蕉 :)
    basket.insert(String::from("banana"), 2);

    // TODO: 在你的水果篮中添加更多的水果。
    basket.insert(String::from("apple"), 4);
    basket.insert(String::from("appl"), 4);
    basket.insert(String::from("app"), 4);
    basket.insert(String::from("ap"), 4);
    basket.insert(String::from("a"), 4);

    basket
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
