fn main() {
    let four = IpAddrKind::V4(String::from("127.0.0.1"));
    let six = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello"));
    m.call();

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // wont compile, x and y different types
    let sum = x + y;
}

enum IpAddrKind {
    V4(String),
    V6(String)
}

enum Message {
    Quit,
    Move { x: i32, y: i32},
    Write (String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {

    }
}