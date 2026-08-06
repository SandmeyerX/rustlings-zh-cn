// `From` 特征用于值到值的转换。如果实现了 `From`，那么会自动提供 `Into` 的实现。
// 你可以在文档中阅读更多关于它的内容:
// https://doc.rust-lang.org/std/convert/trait.From.html

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 我们实现 `Default` 特征，以便在提供的字符串无法转换为 `Person` 对象时将其用作备用方案。
impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

// TODO: 完成这个 `From` 的实现，以便能够从形如 "Mark, 20" 的字符串中解析出一个 `Person`。
// 注意，你需要使用类似 `"4".parse::<u8>()` 的方式将年龄部分解析为 `u8` 类型。
//
// 步骤:
// 1. 根据字符串中存在的逗号对给定字符串进行分割(split)。
// 2. 如果分割操作返回的元素少于或多于2个，返回 `Person` 的默认值。
// 3. 将分割操作得到的第一个元素用作姓名。
// 4. 如果姓名为空，返回 `Person` 的默认值。
// 5. 将分割操作得到的第二个元素解析为 `u8` 类型作为年龄。
// 6. 如果年龄解析失败，返回 `Person` 的默认值。 
impl From<&str> for Person {
    fn from(s: &str) -> Self {}
}

fn main() {
    // 使用 `from` 函数。
    let p1 = Person::from("Mark,20");
    println!("{p1:?}");

    // 由于 `Person` 实现了 `From`，我们也可以使用 `Into`。
    let p2: Person = "Gerald,70".into();
    println!("{p2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let dp = Person::default();
        assert_eq!(dp.name, "John");
        assert_eq!(dp.age, 30);
    }

    #[test]
    fn test_bad_convert() {
        let p = Person::from("");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_good_convert() {
        let p = Person::from("Mark,20");
        assert_eq!(p.name, "Mark");
        assert_eq!(p.age, 20);
    }

    #[test]
    fn test_bad_age() {
        let p = Person::from("Mark,twenty");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_comma_and_age() {
        let p: Person = Person::from("Mark");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_age() {
        let p: Person = Person::from("Mark,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name() {
        let p: Person = Person::from(",1");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_age() {
        let p: Person = Person::from(",");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_missing_name_and_invalid_age() {
        let p: Person = Person::from(",one");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma() {
        let p: Person = Person::from("Mike,32,");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }

    #[test]
    fn test_trailing_comma_and_some_string() {
        let p: Person = Person::from("Mike,32,dog");
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 30);
    }
}
