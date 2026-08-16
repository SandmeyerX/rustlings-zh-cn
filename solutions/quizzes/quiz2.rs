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

    // 使用循环的解决方案。查看 `transformer_iter` 可得到一个使用迭代器的版本。
    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = Vec::new();

        for (string, command) in input {
            // 创建新的字符串。
            let new_string = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            };

            // 将新字符串添加到输出数组。
            output.push(new_string);
        }

        output
    }

    // 等同于 `transform`，但为了便于对比，这里使用迭代器而非循环。
    // 别担心，我们稍后会练习迭代器的哦。
    pub fn transformer_iter(input: Vec<(String, Command)>) -> Vec<String> {
        input
            .into_iter()
            .map(|(string, command)| match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => string + &"bar".repeat(n),
            })
            .collect()
    }
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // 引入 `transformer`。
    use super::my_module::transformer;

    use super::Command;
    use super::my_module::transformer_iter;

    #[test]
    fn it_works() {
        for transformer in [transformer, transformer_iter] {
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
}
