fn main() {
    let mut res = 42;
    let option = Some(12);
    // TODO: 修复此行中的 Clippy lint（检查提示）。
    for x in option {
        res += x;
    }

    println!("{res}");
}
