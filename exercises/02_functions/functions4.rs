// 这家商店正在进行促销活动：
// 如果价格是偶数，可以优惠 10r（Rust 国的货币 —— rua元，简称 r）；
// 如果是奇数，则可以优惠 3r。
// 目前不用担心函数体本身，我们现在只关注函数签名。

fn is_even(num: i64) -> bool {
    num % 2 == 0
}

// TODO: 修复函数签名。
fn sale_price(price: i64) -> {
    if is_even(price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let original_price = 51;
    println!("你的促销价格为 {}", sale_price(original_price));
}
