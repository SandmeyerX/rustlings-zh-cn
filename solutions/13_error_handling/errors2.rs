// 假设我们正在编写一个可以用代币购买道具的游戏。所有道具单价均为 5 个代币，
// 并且每次购买道具都会产生 1 个代币的手续费。
// 玩家会输入他们想要购买的道具数量，由 `total_cost` 函数计算这些道具的总花费。
// 玩家输入的购买数量会以字符串形式获取。但他们可能输入了任何内容，未必是纯数字！
//
// 目前，这个函数根本没有处理错误情况。我们要做的是：
// 当向 `total_cost` 函数传入无法解析为数字的字符串时，该函数返回 `ParseIntError`。
// 在这种情况下，我们希望让函数直接返回该错误，不再执行后续的乘法和加法运算。
//
// 本题至少有两种正确实现方式，其中一种代码会简洁很多！

use std::num::ParseIntError;

#[allow(unused_variables, clippy::question_mark)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // 添加了 `?` 来传播错误。
    let qty = item_quantity.parse::<i32>()?;
    //                                    ^ 已添加

    // 等价于下面这个冗长的版本：
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}
