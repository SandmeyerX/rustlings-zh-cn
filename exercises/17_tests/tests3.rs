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
        // TODO: 这个测试应该检查 `rect` 是否具有我们传递给其构造函数的尺寸大小。
        let rect = Rectangle::new(10, 20);
        assert_eq!(todo!(), 10); // 检查宽（width）
        assert_eq!(todo!(), 20); // 检查高（height）
    }

    // TODO: 这个测试应该检查当我们尝试创建一个宽度为负的矩形时，
    // 程序是否会产生恐慌（panic）。
    #[test]
    fn negative_width() {
        let _rect = Rectangle::new(-10, 10);
    }

    // TODO: 这个测试应该检查当我们尝试创建一个高度为负的矩形时，
    // 程序是否会产生恐慌（panic）。
    #[test]
    fn negative_height() {
        let _rect = Rectangle::new(10, -10);
    }
}
