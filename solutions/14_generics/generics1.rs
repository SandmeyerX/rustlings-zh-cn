// `Vec<T>` 以类型 `T` 为泛型参数。
// 在大多数情况下，编译器能够推断出 `T`，例如在将具有具体类型的值 push 到动态数组之后。
// 但在本练习中，需要通过类型注解来帮助编译器进行推断。

fn main() {
    // `u8` 和 `i8` 都能被转换成 `i16`。
    let mut numbers: Vec<i16> = Vec::new();
    //             ^^^^^^^^^^ 添加了此类型注解

    // 不要修改此行代码。
    let n1: u8 = 42;
    numbers.push(n1.into());
    let n2: i8 = -1;
    numbers.push(n2.into());

    println!("{numbers:?}");
}
