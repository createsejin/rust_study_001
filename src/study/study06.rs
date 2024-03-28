enum IpAddr {
    //V4(String),
    #[allow(dead_code)]
    V4(u8, u8, u8, u8),
    #[allow(dead_code)]
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
    fn call(&self) {}
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

#[derive(Debug)] // 이걸 써야 {:?}를 쓸 수 있나 보다.
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
// 6.2 Patterns That Bind to Values
#[allow(dead_code)]
pub fn study004() {
    let coin1 = Coin::Quarter(UsState::Alabama);
    value_in_cents(coin1);
    let coin2 = Coin::Penny;
    value_in_cents(coin2);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
#[allow(dead_code)]
pub fn study005() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(_num_spaces: u8) {}
fn reroll() {}
#[allow(dead_code)]
pub fn study006() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }
}

// 6.3 Concise Control Flow with if let
#[allow(dead_code)]
pub fn study007() {
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

#[allow(dead_code)]
pub fn study008() {
    let mut count = 0;
    let coin1 = Coin::Quarter(UsState::Alabama);
    // match coin1 {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => count += 1,
    // }
    if let Coin::Quarter(state) = coin1 {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("the counter = {}", count);
}
