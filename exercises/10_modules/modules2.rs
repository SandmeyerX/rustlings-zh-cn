// 你可以使用 `use` 和 `as` 关键字将模块路径引入作用域并为它们取个新名称（别名）。
mod delicious_snacks {
    // TODO: 在修复以下两条 `use` 语句后将它们添加到作用域。
    // use self::fruits::PEAR as ???;
    // use self::veggies::CUCUMBER as ???;

    mod fruits {
        pub const PEAR: &str = "Pear";
        pub const APPLE: &str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &str = "Cucumber";
        pub const CARROT: &str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {}",
        delicious_snacks::fruit,
        delicious_snacks::veggie,
    );
}
