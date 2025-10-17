pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
}

pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}", value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}", value);
        }
        Guess { value }
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}


#[cfg(test)] // 告诉rust 只在执行 cargo test 时才编译和运行测试代码，否则不编译，而在运行 cargo build 时不这么做
mod tests {
    use super::*; // 导入模块中的所有公有项,supper 表示父模块

    #[test] // 这个属性表明这是一个测试函数
    fn it_works_1() {
        println!("it_works===> add(2, 2)");
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works_2() {
        println!("it_works_2===> add(3, 5)");
        let result = add(3, 5);
        assert_eq!(result, 8);
    }

    #[test]
    fn it_works_3() {
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
        assert!(rect1.can_hold(&rect2) == true);
        assert!(rect1.can_hold(&rect3) == false);
        assert_eq!(rect1.can_hold(&rect2), true);
        assert_eq!(rect1.can_hold(&rect3), false);
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was {}", result);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100, got 200")]
    fn greater_than_100_panic() {
        Guess::new(200);
    }

    #[test]
    fn it_works_4() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn it_works_5() {
        println!("it_works_5===> add(4, 6)");
        let result = add(4, 6);
        assert_eq!(result, 10);
    }

    #[test]
    fn add_two_test() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn internal_adder_test() {
        assert_eq!(4, internal_adder(2, 2)); // 可以直接测试私有函数
    }
}

// cargo test --help, --help是cargo test的参数
// 串行执行，cargo test -- --test-threads=1  需要用--分割cargo test 的参数 和 测试文件的参数
// 并行执行，cargo test -- --test-threads=n
// 显示打印，cargo test -- --nocapture
// 显示打印，cargo test -- --show-output
// 运行单个测试，cargo test it_works_4
// 运行多个测试，cargo test it_works // 会进行it_works的匹配
// 忽略测试，cargo test -- --ignored // 只运行ignored的测试