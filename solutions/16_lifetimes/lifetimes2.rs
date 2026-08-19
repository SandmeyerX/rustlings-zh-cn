fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let string1 = String::from("long string is long");

    // 解决方案一:
    // 你可以将 `strings2` 移出内部代码块，这样它就不会在打印语句之前被释放(drop)。
    let string2 = String::from("xyz");
    let result;
    {
        result = longest(&string1, &string2);
    }
    println!("The longest string is '{result}'");
    // `string2` 在函数结束时被释放(drop)。

    // =========================================================================

    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(&string1, &string2);
        // 解决方案二:
        // 你可以将打印语句移到内部代码块中，以便它在 `string2` 被释放之前执行。
        println!("The longest string is '{result}'");
        // `string2` 在这里被释放 (内部作用域结束时)。
    }
}
