// 你可以使用 `use` 关键字将任何模块(尤其是标准库中的模块)的模块路径引入到你的作用域中。

// TODO: 将来自 `std::time` 模块的 `SystemTime` 和 `UNIX_EPOCH` 引入到你的作用域中。
// 如果能用一行代码做到这一点，在代码风格上会加分哦！
// use ???;
use std::time::{SystemTime,UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("1970-01-01 00:00:00 UTC was {} seconds ago!", n.as_secs()),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
