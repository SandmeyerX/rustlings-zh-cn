struct ColorRegularStruct {
    // TODO: 添加测试 `regular_structs` 期望的字段（field）。
    // 这些字段应该是什么类型？RGB 颜色值的最小值和最大值是多少？
}

struct ColorTupleStruct(/* TODO: 添加测试 `tuple_structs` 期望的字段（field） */);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regular_structs() {
        // TODO: 实例化（instantiate）一个普通结构体。
        // let green =

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    #[test]
    fn tuple_structs() {
        // TODO: 实例化一个元组结构体。
        // let green =

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    #[test]
    fn unit_structs() {
        // TODO: 实例化一个单元结构体。
        // let unit_struct =
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }
}
