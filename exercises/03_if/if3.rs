fn animal_habitat(animal: &str) -> &str {
    // TODO: 修复以下语句中的编译器错误。
    let identifier = if animal == "螃蟹" {
        1
    } else if animal == "地鼠" {
        2
    } else if animal == "蟒蛇" {
        3
    } else {
        4
    };

    // 不要修改以下语句。
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
    // (可选)你可以选择性地在此处进行试验。
}

// 不要修改以下测试。
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
