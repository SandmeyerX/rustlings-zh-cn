// 在本练习中，我们有一个名为 `numbers` 的 `u32` 类型的 `Vec`，
// 其中的数值范围从 0 到 99。我们希望将这组数字同时用于 8 个不同的线程。
// 每个线程将计算带有偏移量的每第 8 个值的总和。
//
// 第一个线程（偏移量为 0），将对 0, 8, 16, ... 求和
// 第二个线程（偏移量为 1），将对 1, 9, 17, ... 求和
// 第三个线程（偏移量为 2），将对 2, 10, 18, ... 求和
// ...
// 第八个线程（偏移量为 7），将对 7, 15, 23, ... 求和。
//
// 每个线程都应该持有一个指向该数字动态数组的引用计数指针。但是 `Rc`（引用计数智能指针）不是线程安全的。
// 因此，我们需要使用 `Arc`（Atomic Reference Counting, 原子引用计数智能指针）。
//
// 不要被线程是如何创建（spawn）和等待（join）这些内容分散注意力。
// 我们将在后续关于线程的练习中再进行这方面的练习。

// 不要修改下面几行的代码。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    // TODO: 使用 `Arc` 来定义 `shared_numbers`。
    // let shared_numbers = ???;

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        // TODO: 使用 `shared_numbers` 来定义 `child_numbers`。
        // let child_numbers = ???;

        let handle = thread::spawn(move || {
            let sum: u32 = child_numbers.iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {offset} is {sum}");
        });

        join_handles.push(handle);
    }

    for handle in join_handles.into_iter() {
        handle.join().unwrap();
    }
}
