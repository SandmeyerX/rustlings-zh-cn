// 在上一个练习的基础上，我们希望所有线程都完成它们的工作。
// 但这次，被生成的线程需要负责更新一个共享的值: `JobStatus.jobs_done`。

use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // 如果你想要一个**可变的**共享状态，仅使用 `Arc` 是不够的。
    // 我们需要用 `Mutex` 包装(wrap)值。
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));
    //                    ^^^^^^^^^^^                          ^

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // 在更新共享值之前进行加锁(lock)。
            status_shared.lock().unwrap().jobs_done += 1;
            //           ^^^^^^^^^^^^^^^^
        });
        handles.push(handle);
    }

    // 等待所有任务完成。
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
    //                        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
}
