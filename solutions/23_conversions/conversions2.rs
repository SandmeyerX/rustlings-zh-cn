// `From` 特征用于值到值的转换。如果实现了 `From`，那么会自动提供 `Into` 的实现。
// 你可以在文档中阅读更多关于它的内容:
// https://doc.rust-lang.org/std/convert/trait.From.html
//
// 仙女 Frank 想从地精 Grace 那里购买一些松露，Grace 是一位举世闻名的巧克力大师。
// 松露以 GnomeCoin 计价，但 Frank 只有 FairyCredit。请通过提供 `From` 实现来
// 帮助 Frank 将他的 FairyCredit 转换为 GnomeCoin。按照当前汇率，1 个 FairyCredit
// 价值 100 个 GnomeCoin。

#[derive(Debug)]
struct FairyCredit(u32);

#[derive(Debug, PartialEq)]
struct GnomeCoin(u64);

impl From<FairyCredit> for GnomeCoin {
    fn from(value: FairyCredit) -> Self {
        Self(value.0 as u64 * 100)
    }
}

// 注意，我们不应该提供反向转换：从 GnomeCoin 转换为 FairyCredit。
// 因为小于 100 的 GnomeCoin 无法表示为 FairyCredit，这会使转换有损。
// `From` 特征只适用于无失败且无损的转换。

fn main() {
    // 使用 `from` 函数。
    let g1 = GnomeCoin::from(FairyCredit(12));
    println!("{g1:?}");

    // 由于 `GnomeCoin` 实现了 `From`，我们也可以使用 `Into`。
    let g2: GnomeCoin = FairyCredit(9).into();
    println!("{g2:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let g = GnomeCoin::from(FairyCredit(12));
        assert_eq!(g, GnomeCoin(1200));
    }

    #[test]
    fn test_into() {
        let g: GnomeCoin = FairyCredit(9).into();
        assert_eq!(g, GnomeCoin(900));
    }
}
