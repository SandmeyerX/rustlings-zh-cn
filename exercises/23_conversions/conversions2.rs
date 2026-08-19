// `From` 特征用于值到值的转换。如果实现了 `From`，那么会自动提供 `Into` 的实现。
// 你可以在文档中阅读更多关于它的内容:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// 使用独立的类型来表示不同的度量单位是一种常见的做法。
// 它可以避免意外混淆不同度量单位的数值。

struct Celsius(f64);

struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    // TODO: 将摄氏度转换为华氏度。不用担心浮点
    // 精度。公式是: F = C * 1.8 + 32
}

impl From<Fahrenheit> for Celsius {
    // TODO: 将华氏度转换为摄氏度。
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    const CASES: [(f64, f64); 6] = [
        (-50.0, -58.0),
        (0.0, 32.0),
        (20.0, 68.0),
        (100.0, 212.0),
        (400.0, 752.0),
        (1000.0, 1832.0),
    ];

    #[test]
    fn celsius_to_fahrenheit() {
        for (celsius, fahrenheit) in CASES {
            let Fahrenheit(actual) = Celsius(celsius).into();
            assert_eq!(actual.round(), fahrenheit);
        }
    }

    #[test]
    fn fahrenheit_to_celsius() {
        for (celsius, fahrenheit) in CASES {
            let Celsius(actual) = Fahrenheit(fahrenheit).into();
            assert_eq!(actual.round(), celsius);
        }
    }
}
