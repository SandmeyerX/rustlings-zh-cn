fn main() {
    // (可选)你可以选择性地在此处进行试验。
}

#[cfg(test)]
mod tests {
    // TODO: 仅通过重新排列测试中的代码行来修复编译器错误。
    // 不要添加、更改或删除任何代码行。
    // 例如:
    //     1 | let foo = xxx;  
    //     2 | let bar = www;
    //           重新排列后:
    //     1 | let baz = www;
    //     2 | let foo = xxx;
    #[test]
    fn move_semantics4() {
        let mut x = Vec::new();
        let y = &mut x;
        y.push(42);
        let mut z =  &mut x;
        z.push(13);
        assert_eq!(x, [42, 13]);
    }
}
