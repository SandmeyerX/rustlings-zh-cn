// TODO: 修复此函数上的编译器错误。
fn picky_eater(food: &str) -> &str {
    if food == "草莓" {
        return "美味!";
    } else if food =="土豆"{
       return "还行吧";
    }
   "不吃"
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

// TODO: 阅读这些测试用例，试着理解它们在干什么。
// 在不修改测试用例的情况下，使所有测试通过。
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // 这说明在调用`picky_eater`函数时，传入参数`草莓`应该返回`美味!`。
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
