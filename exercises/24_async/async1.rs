// Alice 是一名小学老师，她需要计算自己所教三个班级的考试平均分。
// 她不打算逐一计算，而是决定请朋友 Bob 和 Catherine 来帮忙。
// 大家分工协作，能更快完成这项工作。
//
// 让我们用异步编程来模拟这个过程。
// 每个人都被表示为一个异步任务，可以并发执行。

// 异步任务需要由"运行时"来执行，而 Rust 的标准库中并不提供运行时。
// 这里我们使用主流的运行时库 `tokio`。
// `tokio::main` 宏将整个 main 函数包装在一个运行时中。
#[tokio::main]
async fn main() {
    let mean_score_a = tokio::spawn(calculate_mean_score("scores_class_a.txt"));
    let mean_score_b = tokio::spawn(calculate_mean_score("scores_class_b.txt"));
    let mean_score_c = tokio::spawn(calculate_mean_score("scores_class_c.txt"));

    // TODO: 等待生成（spawn）的任务以检查它们的结果。
    assert_eq!(mean_score_a, 84); // alice
    assert_eq!(mean_score_b, 89); // bob
    assert_eq!(mean_score_c, 76); // catherine
}

// TODO: 将生成 (spawn) 的函数改为异步函数，以修复编译错误。
fn calculate_mean_score(scores_file: &str) -> usize {
    // 异步地读取文件
    let file = tokio::fs::read_to_string(scores_file).await.unwrap();

    // 初始化总分与分数计数
    let mut sum = 0;
    let mut n = 0;
    for line in file.lines() {
        // 将每一行解析为一个分数
        let score = line.parse::<usize>().unwrap();
        sum += score;
        n += 1;
    }

    sum / n
}
