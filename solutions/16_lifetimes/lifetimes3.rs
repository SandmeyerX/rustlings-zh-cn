// 当结构体持有引用时，同样需要生命周期。

struct Book<'a> {
    //     ^^^^ 添加生命周期注解
    author: &'a str,
    //       ^^
    title: &'a str,
    //      ^^
}

fn main() {
    let book = Book {
        author: "George Orwell",
        title: "1984",
    };

    println!("{} by {}", book.title, book.author);
}
