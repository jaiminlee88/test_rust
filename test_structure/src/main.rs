#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn test1() {
    // #[derive(Debug)]
    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }
    println!("test1 start==================");
    let user1 = User {
        username: String::from("user1"),
        email: String::from("user1@example.com"),
        sign_in_count: 1,
        active: true,
    };
    println!("{} {} {} {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("user1={:?}", user1);
}

fn build_user(username: String, email: String) -> User {
    User {
        username, // 参数名与结构体字段名相同，可以简写
        email,
        sign_in_count: 1,
        active: true,
    }
}
fn test2() {
    println!("test2 start==================");
    let user2 = build_user(String::from("user2"), String::from("user2@example.com"));
    println!("{} {} {} {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    println!("user2={:?}", user2);
}


fn test3() {
    println!("test3 start==================");
    let user3 = build_user(String::from("user3"), String::from("user3@example.com"));
    let user4 = User {
        username: String::from("user4"),
        email: String::from("user4@example.com"),
        ..user3 // 使用user3的其余字段，user3的其余字段会自动填充
    };
    println!("user3={:?}", user3);
    println!("user4={:?}", user4);
}

#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

fn test4() {
    println!("test4 start==================");
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black=({},{},{})", black.0, black.1, black.2);
    println!("origin=({},{},{})", origin.0, origin.1, origin.2);
}


fn test5() {
    println!("test5 start==================");
    // struct UserRef {
    //     username: &String,// 必须显示声明生命周期
    //     email: &String,// 必须显示声明生命周期
    //     sign_in_count: u64,
    //     active: bool,
    // }
    struct UserRef<'a> {
        username: &'a String,
        email: &'a String,
        sign_in_count: u64,
        active: bool,
    }
    let username1 = String::from("user5");
    let email1 = String::from("user5@example.com");
    let user5 = UserRef {
        username: &username1, // 生命周期离开后就失效，
        email: &email1,
        sign_in_count: 1,
        active: true,
    };
    println!("user5={},{},{},{}", user5.username, user5.email, user5.sign_in_count, user5.active);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
struct Rect {
    width: u32,
    height: u32,
}
fn area2(rect: &Rect) -> u32 {
    rect.width * rect.height
}
fn test6() {
    println!("test6 start==================");
    let rect1 = (30, 50); // 元组
    let area1 = area(rect1); // 元组传参
    println!("area1={}", area1);

    let rect2 = Rect {
        width: 40,
        height: 50,
    };
    let area2 = area2(&rect2); // 结构体传参
    println!("area2={}", area2);
}

fn test7() {
    println!("test7 start==================");
    let rect3 = Rect {
        width: 50,
        height: 60,
    };
    impl Rect {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    println!("rect3={}", rect3.area());
}

fn main() {
    test1(); // structure
    test2(); // 
    test3(); // 
    test4(); // 元组与结构体
    test5(); // 结构体与lifetime
    test6(); // 元组与结构体
    test7(); // 派生trait
}
