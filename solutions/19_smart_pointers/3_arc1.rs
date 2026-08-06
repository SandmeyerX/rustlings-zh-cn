// 在本练习中，我们有一个名为`numbers`的`u32`类型的 `Vec`，
// 其值范围是从0到99。我们希望能在8个不同的线程中同时使用这组数字。
// 每个线程将要获取每隔八个值(带有偏移量(offset))的总和。
//
// 第一个线程(偏移量为0)，将会对 0, 8, 16, ... 这些数字求和
// 第二个线程(偏移量为1)，将会对 1, 9, 17, ... 这些数字求和
// 第三个线程(偏移量为2)，将会对 2, 10, 18, ... 这些数字求和
// ...
// 第八个线程(偏移量为7)，将会对 7、15、23……这些数字求和。
//
// 每个线程都应该拥有一个指向数字动态数组的引用计数指针。但是`Rc`(引用计数智能指针)不是线程安全的。
// 因此，我们需要使用`Arc`(Atomic Reference Counting, 原子引用计数智能指针)。
//
// 不要被线程是如何生成(spawn)和汇合(join)这些内容分散注意力。
// 我们将在后续关于线程的练习中再进行这方面的练习。 

// 不要修改下面几行的代码。
#![forbid(unused_imports)]
use std::{sync::Arc, thread};

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();

    let shared_numbers = Arc::new(numbers);
    //                   ^^^^^^^^^^^^^^^^^

    let mut join_handles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers);
        //                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^

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
