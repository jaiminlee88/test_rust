
fn add_one(x: i32) -> i32 {
    x + 1
}
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}
fn test1() {
    println!("========== test1 ==========");
    println!("do_twice(add_one, 5) = {}", do_twice(add_one, 5));
}

fn test2() {
    println!("========== test2 ==========");
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings_1: Vec<String> = list_of_numbers
    .iter()
    .map(|i| i.to_string()) // 使用闭包
    .collect();
    println!("list_of_strings_1 = {:?}", list_of_strings_1);

    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings_2: Vec<String> = list_of_numbers
    .iter()
    .map(ToString::to_string) // 使用函数
    .collect();
    println!("list_of_strings_2 = {:?}", list_of_strings_2);
}

fn return_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x: i32| -> i32 { x + 1 })
}
fn test3() {
    println!("========== test3 ==========");
    let add_one = return_closure();
    println!("add_one(5) = {}", add_one(5));
}
fn main() {
    test1(); // 函数指针
    test2(); // 闭包
    test3(); // 返回闭包
}
