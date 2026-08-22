struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    // 不要修改此函数。
    fn new(width: i32, height: i32) -> Self {
        if width <= 0 || height <= 0 {
            // 这里返回一个 `Result` 类型会更好些。
            // 但我们想要学习如何测试可能会产生恐慌（panic）的函数。
            panic!("Rectangle width and height must be positive");
        }

        Rectangle { width, height }
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_width_and_height() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.width, 10); // 检查宽（width）
        assert_eq!(rect.height, 20); // 检查高（height）
    }

    #[test]
    #[should_panic] // 添加此属性（attribute）来检查测试是否会产生恐慌（panic）。
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    #[test]
    #[should_panic] // 添加此属性（attribute）来检查测试是否会产生恐慌（panic）。
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
