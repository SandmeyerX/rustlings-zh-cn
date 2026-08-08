fn picky_eater(food: &str) -> &str {
    if food == "草莓" {
        "美味!"
    } else if food == "土豆" {
        "还行吧"
    } else {
        "不吃"
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        assert_eq!(picky_eater("草莓"), "美味!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("土豆"), "还行吧");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("香菜"), "不吃");
        assert_eq!(picky_eater("洋葱"), "不吃");
        assert_eq!(picky_eater("大蒜"), "不吃");
    }
}
