// 本练习是 `errors4` 练习的一个变形版本。
// 它用到了一些在课程后续才会学到的概念，比如 `Box` 和 `From` trait。
// 目前你无需详细理解这些概念，但如果你愿意，也可以提前了解一下。
// 现阶段，你可以将 `Box<dyn ???>` 类型理解为“任意实现了 ??? (trait) 的东西”。
//
// 简而言之，此处“Box”的使用场景为：当你想要持有一个值，
// 且只关心该类型是否实现了某个特定特征时。
// 要做到这一点，可将 `Box` 声明为 `Box<dyn Trait>` 类型，
// 其中 `Trait` 就是编译器会为此场景下所有使用的值所匹配的特征。
// 对于本练习而言，该场景特指 `Result` 中可能返回的各类错误。

use std::error::Error;
use std::fmt;

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

// 这样做是为了让 `CreationError` 能够实现 `Error`。
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl Error for CreationError {}

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}
