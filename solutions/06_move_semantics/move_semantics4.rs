fn main() {
    // 你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // 仅通过重新排列测试中的代码行来修复编译器错误。
    // 不要添加、更改或删除任何代码行。
    // 例如：
    //     1 | let foo = xxx;
    //     2 | let bar = www;
    //           重新排列后：
    //     1 | let bar = www;
    //     2 | let foo = xxx;
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        // `y` 在这里被使用了。
        y.push(42);
        // 可变引用 `y` 不再被使用了，
        // 因此可以创建一个新的引用。
        let z = &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
