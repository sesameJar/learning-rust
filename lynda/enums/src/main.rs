enum IpAddrKind {
    v4(String),
    v6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("Im inside call");
    }
}

fn main() {
    let four = IpAddrKind::v4;
    let six = IpAddrKind::v6;

    let home = IpAddrKind::v4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::v6(String::from("::1"));

    let m = Message::Write(String::from("Hello"));

    m.call();
}
