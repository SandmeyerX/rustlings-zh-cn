// 这个强大的包装器（wrapper）具备存储正整数值的能力。
// TODO: 使用泛型重写它，使其能支持包装任意类型。
struct Wrapper {
    value: u32,
}

// TODO: 调整结构体的实现（impl），使其以被包装值的类型为泛型参数。
impl Wrapper {
    fn new(value: u32) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
