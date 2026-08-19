#[derive(Debug)]
enum Message {
    // TODO: 定义下面所使用的几种消息（Message）类型。
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
