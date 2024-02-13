enum IpAddr {
    //V4(String),
    V4(u8, u8, u8, u8),
    V6(String),
}
#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {

    }
}

#[allow(dead_code)]
pub fn study001() {
    //let home = IpAddr::V4(String::from("127.0.0.1"));
    let _home = IpAddr::V4(127, 0, 0, 1);
    let _loopback = IpAddr::V6(String::from("::1"));
}
#[allow(dead_code)]
pub fn study002() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
#[allow(dead_code)]
pub fn study003() {
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}
#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// 6.2 Patterns That Bind to Values
#[allow(dead_code)]
pub fn study004() {
    
}