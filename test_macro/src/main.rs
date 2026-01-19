use test_macro::myvec;
use hello_derive::Hello; // 派生宏引入到当前工程，编译器依赖

fn test1() {
    println!("========== test1 ==========");
    let v = myvec![1, 2, 3];
    println!("v = {:?}", v);
}

#[derive(Hello)]
struct User;

fn test2() {
    println!("========== test2 ==========");
    User::hello(); // 调用派生宏生成的 hello 方法，插入到此
}
fn main() {
    test1(); // 声明式宏
    test2(); // 过程宏之派生宏
}
