// 结构体(Structs)包含数据(data)，但也可以包含逻辑(logic)。
// 在这个练习中，我们定义了 `Fireworks` 结构体，以及一些处理它的函数。
// 把这些独立函数转换为方法和关联函数，
// 以便在代码中更清晰地表达这种关系。

#![deny(clippy::use_self)] // 练习使用 `Self` 类型

#[derive(Debug)]
struct Fireworks {
    rockets: usize,
}

// TODO: 把这个函数转换为 `Fireworks` 的关联函数。
fn new_fireworks() -> Fireworks {
    Fireworks { rockets: 0 }
}

// TODO: 把这个函数转换为 `Fireworks` 的方法。
fn add_rockets(fireworks: &mut Fireworks, rockets: usize) {
    fireworks.rockets += rockets
}

// TODO: 把这个函数转换为 `Fireworks` 的方法。
fn start(fireworks: Fireworks) -> String {
    if fireworks.rockets < 5 {
        String::from("small")
    } else if fireworks.rockets < 20 {
        String::from("medium")
    } else {
        String::from("big")
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
