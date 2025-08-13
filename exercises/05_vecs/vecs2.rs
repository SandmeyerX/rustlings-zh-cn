fn vec_loop(input: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();

    for element in input {
        // TODO: 将 `input` 切片中的每个都元素乘以2，
        // 并将其添加(push)到 `output` 动态数组中。
        output.push(element*2);
    }

    output
}

fn vec_map_example(input: &[i32]) -> Vec<i32> {
    // 下面是一个先对元素进行映射(map)，然后收集(collect)成动态数组的例子。
    // 我们先把`input`切片里的每个元素都变成(映射)其原本的值加1 。
    //         1 -> 1 + 1 => 2
    //         2 -> 2 + 1 => 3
    //         3 -> 3 + 1 => 4
    //         n -> n + 1 => n + 1
    // 例如，如果输入的是 `[1, 2, 3]` ，那得到的输出就会是 `[2, 3, 4]` 。 
    input.iter().map(|element| element + 1).collect()
}

fn vec_map(input: &[i32]) -> Vec<i32> {
    // TODO: 这里，我们同样想要将 `input` 切片中的每个元素都乘以2，
    // 但这次要使用迭代器映射的方式，而不是手动将元素逐个添加到一个空的动态数组中。
    // 可参考上面 `vec_map_example` 函数中的示例。 
    input
        .iter()
        .map(|element| {
            element*2
        })
        .collect()
        
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
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

    #[test]
    fn test_vec_map_example() {
        let input = [1, 2, 3];
        let ans = vec_map_example(&input);
        assert_eq!(ans, [2, 3, 4]);
    }

    #[test]
    fn test_vec_map() {
        let input = [2, 4, 6, 8, 10];
        let ans = vec_map(&input);
        assert_eq!(ans, [4, 8, 12, 16, 20]);
    }
}
