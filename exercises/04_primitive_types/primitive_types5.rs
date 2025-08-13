fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: 在一条语句中解构 `cat` 元组，使得 `println` 语句能够正常工作。
    // let /* 此处为你的模式语句 */ = cat;
    let (name,age) = cat;
    println!("{name} 今年 {age} 岁了");
}
