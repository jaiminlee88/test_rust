use std::thread;
use std::time::Duration;


fn generate_workout(intensity: u32, random_number: u32) {
    /*
    闭包的定义：
    |param1, param2, ...| {
        // 闭包体
    }
    - 闭包的参数和返回值类型可以省略，如果省略，则需要使用 -> 显式声明返回值类型
    - 在一个作用域内，闭包
    */
    
    // 合法闭包定义1
    // let expensive_closure = |num: u32| -> u32 { // 闭包定义，接收一个参数num，返回一个u32，如果多个参数，逗号分隔        println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(1));
    //     num
    // };
    
    // 合法闭包定义2
    let expensive_closure = |num| { // 合法闭包定义
        thread::sleep(Duration::from_secs(1));
        num
    };

    expensive_closure(32); // 第一次调用，参数类型和返回类型会被锁定进闭包，再次调用时必须保持一致
    // expensive_closure(String::from("Hello")); // 此时第二次调用，会报错，因为闭包的参数类型已经定义为u32，不能传入String类型
    
    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure(intensity));
        }
    }
}

fn test1() {
    println!("=============== test1 ===============");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}


struct Cacher<T> // trait bound给泛型加限制，让它变得可用。
    where T : Fn(u32) -> u32 //  T的trait bound指定了T是一个使用Fn的闭包，告诉编译器泛型参数 T 必须实现某个 trait，才能在这个类型或函数中被使用。
{
    calculation: T, // trait bound要求这个calculation字段必须实现Fn trait
    value: Option<u32>, // 可用some(v)或none来表示有值或没有值
}

impl<T> Cacher<T>
    where T : Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher { calculation, value: None } // new时装入了一个闭包，但没有执行
    }
    fn value(&mut self, arg:u32) -> u32 { // mut关键字表示可变引用，因为self是可变引用
        match self.value {
            Some(v) => v, // 如果value有值，则直接返回，所以只计算了一次
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout_using_cacher(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}

fn test2() {
    println!("=============== test2 ===============");
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_using_cacher(simulated_user_specified_value, simulated_random_number);
}

fn test3() {
    println!("=============== test3 ===============");
    let x = 4;
    let equal_to_x = |z| { z == x }; // 捕捉了x，并存储在闭包中，有一定成本
    let y = 4;
    assert!(equal_to_x(y));
}

fn test4() {
    println!("=============== test4 ===============");
    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x; // x所有权转移给闭包，所以x不能再次使用
    // println!("x is moved to closure: {}", x); // 这行会报错，因为x已经被移动到闭包中了
    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    stype: String,
}
fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s| {s.size == shoe_size})
        .collect() // 消耗迭代器，返回新的vec
}
fn test5() {
    println!("=============== test5 ===============");
    let shoes = vec![
        Shoe { size: 10, stype: String::from("sneaker") },
        Shoe { size: 13, stype: String::from("sandal") },
        Shoe { size: 10, stype: String::from("boot") },
    ];
    let in_my_size = shoes_in_my_size(shoes, 10);
    for shoe in in_my_size {
        println!("shoe: {:?}", shoe);
    }
    // println!("in_my_size: {:?}", in_my_size);
    // println!("shoes: {:?}", shoes); // shoes所有权转移给闭包，所以shoes不能再使用
}

fn main() {
    println!("Hello, world!");
    // test1(); // 测试闭包的定义和调用
    test2(); // memoization 或 lazy evaluation
    test3(); // 闭包捕获环境
    test4(); // 闭包捕获环境，move关键字转移所有权
    test5(); // 闭包捕获环境，使用迭代器过滤数据
}
