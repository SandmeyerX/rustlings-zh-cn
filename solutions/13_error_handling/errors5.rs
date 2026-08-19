// 本练习是 `errors4` 练习的一个变形版本。
// 它使用了一些我们在课程后面才会学到的概念，比如 `Box` 以及 `From` trait。
// 现在没必要详细理解它们，不过如果你愿意的话可以提前阅读相关内容。
// 目前，可以把 `Box<dyn???>` 类型看作是一种 `我想要任何都能实现??? 功能` 这样的类型。
//
// 简而言之，这种针对“Box”的特定用例适用于当你想要持有一个值，
// 并且你只关心它是一个实现了特定trait(特征)的类型这种情况。
// 要做到这一点，`Box` 会被声明为 `Box<dyn Trait>` 类型，
// 其中 `Trait` 就是编译器在该上下文中所使用的任何值上查找的特征。
// 对于本练习而言，这个上下文就是那些可能在 `Result` 中返回的错误情况。

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
