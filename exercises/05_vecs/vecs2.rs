fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: 将 `input` 切片中的每个元素都乘以 2，
        // 并将其 push 到 `output` 动态数组中。
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // NOTE: `vec_map_example` 和 `vec_map` 练习已在 [Commit d8f4b06] 中被官方移除，
    //     原因是学生此时尚不具备理解迭代器的相关知识，该主题将在后续专门的迭代器练习中详细讲解。
    //     本中文版本保留了这两个练习，你可以自行选择是否完成。
    // See: https://github.com/rust-lang/rustlings/issues/2102

    // 下面是一个先对元素进行映射（map），然后将结果收集（collect）成动态数组的例子。
    // 我们将 `input` 切片中的每个元素都映射为它原本的值加 1：
    //         1 -> 1 + 1 => 2
    //         2 -> 2 + 1 => 3
    //         3 -> 3 + 1 => 4
    //         n -> n + 1 => n + 1
    // 例如，如果输入是 `[1, 2, 3]`，那么输出就是 `[2, 3, 4]`。
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: 这里我们同样想把 `input` 切片中的每个元素都乘以 2，
    // 但这次改用迭代器映射（map）的方式，而不是手动向空的动态数组中逐个添加元素。
    // 可以参考上面 `vec_map_example` 函数中的示例。
    input
        .iter()
        .map(|element| {
            // ???
        })
        .collect()
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_loop(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }

    #[ignore = "在[Commit d8f4b06]中，此练习已被移除"]
    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[ignore = "在[Commit d8f4b06]中，此练习已被移除"]
    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
