// 假设我们正在编写一个游戏，在这个游戏中你可以用代币购买物品。所有物品都花费5个代币，
// 并且每当你购买物品时会有1个代币的手续费。
// 游戏玩家将输入他们想要购买的物品数量，而 `total_cost` 函数将计算这些物品的总成本。
// 由于玩家输入的是数量，所以我们得到的是一个字符串。他们可能输入了任何东西，而不仅仅是数字！
//
// 目前，这个函数根本没有处理错误情况。我们要做的是:
// 如果我们在一个不是数字的字符串上调用 `total_cost` 函数，该函数将返回一个 `ParseIntError`。
// 在这种情况下，我们希望立即从我们的函数中返回该错误，而不是尝试进行乘法和加法运算。
//
// 至少有两种实现方式都是正确的。但其中一种要简短得多！

use std::num::ParseIntError;

#[allow(unused_variables, clippy::question_mark)]
fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Added `?` to propagate the error.
    let qty = item_quantity.parse::<i32>()?;
    //                                    ^ added

    // Equivalent to this verbose version:
    let qty = match item_quantity.parse::<i32>() {
        Ok(v) => v,
        Err(e) => return Err(e),
    };

    Ok(qty * cost_per_item + processing_fee)
}

fn main() {
    // (可选)你可以选择性地在此处进行试验。
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
