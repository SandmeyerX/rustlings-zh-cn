// `AsRef` 和 `AsMut` 允许进行低成本的引用到引用的转换。
// 分别在 https://doc.rust-lang.org/std/convert/trait.AsRef.html 
// 和 https://doc.rust-lang.org/std/convert/trait.AsMut.html 
// 阅读更多关于它们的内容。 

// 获取给定参数中的字节数(bytes, 而非字符数)
// (`.len()` 方法返回的是字符串中的字节数)。
fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().len()
}

// 获取给定参数中的字符数(而非字节数)。
fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

// 使用 `as_mut()` 并对一个数字进行求平方操作。
fn num_sq<T: AsMut<u32>>(arg: &mut T) {
    let arg = arg.as_mut();
    *arg *= *arg;
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn different_counts() {
        let s = "Café au lait";
        assert_ne!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn same_counts() {
        let s = "Cafe au lait";
        assert_eq!(char_counter(s), byte_counter(s));
    }

    #[test]
    fn different_counts_using_string() {
        let s = String::from("Café au lait");
        assert_ne!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn same_counts_using_string() {
        let s = String::from("Cafe au lait");
        assert_eq!(char_counter(s.clone()), byte_counter(s));
    }

    #[test]
    fn mut_box() {
        let mut num: Box<u32> = Box::new(3);
        num_sq(&mut num);
        assert_eq!(*num, 9);
    }
}
