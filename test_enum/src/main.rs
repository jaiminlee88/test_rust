#[derive(Debug)]
enum IpAddrKind {
    V4, // 无参
    V6, // 有参
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}


fn test1() {
    println!("test1 start==================");
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("home={:?}", home);
    println!("loopback={:?}", loopback);
}

fn test2() {
    println!("test2 start==================");
    enum Message {
        Quit, // 无任何数据关联
        Move { x: i32, y: i32 }, // 匿名结构体
        Write(String), // 元组
        ChangeColor(i32, i32, i32), // 元组
    }

    impl Message {
        fn call(&self) {
            println!("call");
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
}

#[derive(Debug)]
enum Option<T> {
    Some(T),
    None,
}

fn test3() {
    println!("test3 start==================");
    let some_number = Option::Some(5);
    let some_char = Option::Some('e');
    let absent_number: Option<i32> = Option::None;
    println!("some_number={:?}", some_number);
    println!("some_char={:?}", some_char);
    println!("absent_number={:?}", absent_number);
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => {
            println!("Nickel");
            5
        }
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from {:?}", state);
            25
        }
    }
}
fn test4() {
    println!("test4 start==================");
    let coin = Coin::Quarter(UsState::Alaska); // 枚举可以有参数
    println!("value_in_cents={}", value_in_cents(coin));
}

fn plus_one(x: std::option::Option<i32>) -> std::option::Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn test5() {
    println!("test5 start==================");
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six={:?}", six);
    println!("none={:?}", none);
}

fn test6() {
    println!("test6 start==================");
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        _ => (),
    }
}

fn test7() {
    println!("test7 start==================");
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    } else {
        println!("other");
    }
}
fn main() {
    test1(); // enum
    test2(); // enum message
    test3(); // enum option
    test4(); // enum match
    test5(); // enum match, plus, option
    test6(); // enum match, plus, option
    test7(); // if let
}
