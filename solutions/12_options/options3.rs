#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // 解决方案一:
    // 对 `Option`(而非 `&Option`)进行模式匹配，但不从 `Some` 变体中移出值。
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
        //   ^^^ added
        _ => panic!("No match!"),
    }

    // 解决方案二:
    // 通过在 `optional_point` 前添加 `&` 来对引用(`&Option`)进行模式匹配。
    match &optional_point {
        Some(p) => println!("Coordinates are {},{}", p.x, p.y),
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}");
}
