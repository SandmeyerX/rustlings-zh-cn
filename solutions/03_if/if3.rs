fn animal_habitat(animal: &str) -> &str {
    let identifier = if animal == "螃蟹" {
        1
    } else if animal == "地鼠" {
        // 整数，这样每个分支都有相同的类型。
        2
    } else if animal == "蟒蛇" {
        3
    } else {
        // 任何未使用的标识符。
        4
    };

    // 在 Rust 中，你会使用枚举（enum）来替代这样的标识符。
    // 不过我们还没学到枚举呢。
    if identifier == 1 {
        "海滩"
    } else if identifier == 2 {
        "地洞"
    } else if identifier == 3 {
        "沙漠"
    } else {
        "未知"
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("地鼠"), "地洞")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("蟒蛇"), "沙漠")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("螃蟹"), "海滩")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("恐龙"), "未知")
    }
}
