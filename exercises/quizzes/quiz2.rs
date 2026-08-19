
// 这是针对以下章节内容的一个小测验：
// - 字符串（Strings）
// - 动态数组（Vecs）
// - 移动语义（Move semantics）
// - 模块（Modules）
// - 枚举（Enums）
//
// 让我们以函数的形式构建一个小型机器。
// 作为输入，我们将给出一个字符串列表以及命令列表。这些命令将决定要对字符串执行何种操作。
// 操作可能是以下几种情况：
// - 将字符串转换为大写形式
// - 去除字符串两端的空白字符
// - 将字符串按指定的次数追加“bar”

// 确定的情况如下：
// - 输入将会是一个由长度为 2 的元组构成的动态数组，
//   其中第一个元素是字符串，第二个元素是命令。
// - 输出将是一个由字符串组成的动态数组。

enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // TODO: 按照上述要求实现该函数。
    // pub fn transformer(input: ???) -> ??? { ??? }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // TODO: 我们需要引入什么才能使 `transformer` 在作用域内可用呢？
    // use ???;
    use super::Command;

    #[test]
    fn it_works() {
        let input = vec![
            ("hello".to_string(), Command::Uppercase),
            (" all roads lead to rome! ".to_string(), Command::Trim),
            ("foo".to_string(), Command::Append(1)),
            ("bar".to_string(), Command::Append(5)),
        ];
        let output = transformer(input);

        assert_eq!(
            output,
            [
                "HELLO",
                "all roads lead to rome!",
                "foobar",
                "barbarbarbarbarbar",
            ]
        );
    }
}
