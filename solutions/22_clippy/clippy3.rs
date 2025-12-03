use std::mem;

#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<&str> = None;
    // 在检查 `Option` 是否为 `None` 之后再对其进行 `unwrap` 操作将会导致程序崩溃。
    // 请改用 `if-let` 语句来替代。
    if let Some(value) = my_option {
        println!("{value}");
    }

    // 缺少了一个逗号。
    #[rustfmt::skip]
    let my_arr = &[
        -1, -2, -3,
        -4, -5, -6,
    ];
    println!("My array! Here it is: {my_arr:?}");

    let mut my_vec = vec![1, 2, 3, 4, 5];
    // `resize` 会改变一个动态数组，而不是返回一个新的。
    // `resize(0, …)` 会清空一个动态数组，所以最好使用 `clear` 来进行清空操作。
    my_vec.clear();
    println!("This Vec is empty, see? {my_vec:?}");

    let mut value_a = 45;
    let mut value_b = 66;
    // 使用 `mem::swap` 来正确地交换两个值。
    mem::swap(&mut value_a, &mut value_b);
    println!("value a: {value_a}; value b: {value_b}");
}
