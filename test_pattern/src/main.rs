
fn test1() {
    println!("===========test1===========");
    enum Message {
        Hello {id: i32},
    }
    // 范围取值 x @ 1..=10
    // let msg = Message::Hello {id: 5};
    let msg = Message::Hello {id: 11};
    match msg {
        Message::Hello {id : id_variable @3..7} => { // id落在[3,7)之间，id值绑定到id_variable
            println!("Found an id in range: {}!", id_variable)
        }
        Message::Hello {id : 10..=12} => {
            println!("Found an id in another range") // 没有绑定，拿不到id值
        }
        Message::Hello {id} => {
            println!("Found some other id: {}!", id);
        }
    }
}

fn test2() {
    println!("===========test2===========");
    let num = Some(4);
    match num {
        Some(x) if x < 5 => println!("The number {} is less than 5", x), // 从左到右逐个匹配
        Some(x) => println!("The number {} is greater than or equal to 5", x),
        None => println!("The number is None"),
    }
}

fn main() {
    test1(); // bind
    test2(); // match guard
}
