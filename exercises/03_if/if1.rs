fn bigger(a: i32, b: i32) -> i32 {
    // TODO: 完善这个函数，满足以下要求:
    //
    // 返回`a`, `b`中最大的数字
    // 如果两个数字相等，返回它们之中的任何一个都可以。
    // 不要使用：
    // - 另一个函数调用
    // - 额外的变量(仅能使用`a`和`b`)
    if a>b {
        return a
    }
    else if a<b {
        return b
    }
    return a;

}

fn main() {
   let a =10;
   let b =5;
   let c=  bigger(a,b);
   println!("max {}",c);
}

// 现在先暂时不要关注下面的代码 :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
