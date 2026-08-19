// 字符类型 (`char`)

fn main() {
    // 注意是单！引！号！（''），与你之前看到的双引号（""）有所不同。
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("是字母!");
    } else if my_first_initial.is_numeric() {
        println!("是数字!");
    } else {
        println!("既不是字母，也不是数字!");
    }

    // TODO: 与之前的例子类似，声明一个名为 `your_character` 的变量，
    // 给它赋予一个你喜欢的字符。
    // 可以尝试用一个字母、一个数字（要用单引号括起来）、一个特殊字符、一个来自与你母语不同语言的字符，
    // 或者一个 emoji 😉
    // let your_character = '';

    if your_character.is_alphabetic() {
        println!("是字母!");
    } else if your_character.is_numeric() {
        println!("是数字!");
    } else {
        println!("既不是字母，也不是数字!");
    }
}
