#![deny(clippy::use_self)] // 练习使用 `Self` 类型

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

impl Fireworks {
    fn new() -> Self {
        Self { rockets: 0 }
    }

    fn add_rockets(&mut self, rockets: usize) {
        self.rockets += rockets
    }

    fn start(self) -> String {
        if self.rockets < 5 {
            String::from("small")
        } else if self.rockets < 20 {
            String::from("medium")
        } else {
            String::from("big")
        }
    }
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn start_some_fireworks() {
        let mut f = Fireworks::new();
        f.add_rockets(3);
        assert_eq!(f.start(), "small");

        let mut f = Fireworks::new();
        f.add_rockets(15);
        assert_eq!(f.start(), "medium");

        let mut f = Fireworks::new();
        f.add_rockets(100);
        // 在最后一个测试中我们不使用方法语法，以确保 `start`
        // 函数取得烟花的所有权。
        assert_eq!(Fireworks::start(f), "big");
    }
}
