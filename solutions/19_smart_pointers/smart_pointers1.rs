// 在编译时期，Rust需要知道一个类型占用多少空间。对于递归类型来说，这就成了问题，
// 因为在递归类型中，一个值可以将同类型的另一个值作为自身的一部分。
// 为解决这个问题，我们可以使用`Box` —— 一种用于在堆上存储数据的智能指针，
// 它还允许我们包装递归类型。
//
// 我们在本练习中要实现的递归类型是“构造列表(cons list)”，
// 这是一种在函数式编程语言中常见的数据结构。
// 构造列表中的每一项包含两个元素: 当前项的值以及下一项，最后一项是一个被称为`Nil`的值。 

#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn create_empty_list() -> List {
    List::Nil
}

fn create_non_empty_list() -> List {
    List::Cons(42, Box::new(List::Nil))
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
