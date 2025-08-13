// 这家商店正在进行促销活动，
// 如果价格是偶数，你可以获得10r(Rust元)的折扣，
// 但如果价格是奇数，则可以获得3r的折扣。
// 目前不用担心函数体本身，我们现在只对函数签名感兴趣。

fn is_even(num:  &i64) -> bool {
    num % 2 == 0
}

// TODO: 修改函数签名。
fn sale_price(price: i64) ->  i64 {
    if is_even(&price) {
        price - 10
    } else {
        price - 3
    }
}

fn main() {
    let  original_price = 51;
    println!("你的促销价格为 {}", sale_price(original_price));
}
