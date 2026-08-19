fn square(num: i32) -> i32 {
    // 已移除下面这行代码末尾的分号 `;`，以便隐式返回结果。
    num * num
}

fn main() {
    let answer = square(3);
    println!("The square of 3 is {answer}");
}
