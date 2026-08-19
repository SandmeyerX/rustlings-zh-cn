// 在Rust中，类型转换是通过使用 `as` 运算符来完成的。
// 请注意，`as` 运算符并非仅在类型转换时使用，它还有助于对引入(use ??? as)的内容进行重命名。

fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
    //                   ^^^^^^
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_proper_type_and_value() {
        assert_eq!(average(&[3.5, 0.3, 13.0, 11.7]), 7.125);
    }
}
