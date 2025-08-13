#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let optional_point = Some(Point { x: 100, y: 200 });

    // TODO: 通过向这个 `match` 语句中添加一些东西来修复编译器错误。
    match optional_point {
        Some(ref p) => println!("Coordinates are {},{}", p.x, p.y),
     
        _ => panic!("No match!"),
    }

    println!("{optional_point:?}"); // 不要修改此行代码。
}
