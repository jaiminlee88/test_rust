

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

fn test1() {
    println!("test1 start==================");
    let sum = add(1, 2);
    println!("sum={}", sum);
}

fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}
fn plus_two(x: i32) -> i32 {
    return x + 2;
}
fn test2() {
    println!("test2 start==================");
    let x = 6;
    let y = {
        let x = 3;
        x + 1 // 表达式，不需要分号
    };
    println!("y={}", y);
    let x = five();
    println!("set x={}", x);
    let x = plus_one(x);
    println!("plus_one x={}", x);
    let x = plus_two(x);
    println!("plus_two x={}", x);
}

fn main() {
    test1(); // 函数
    test2(); // 表达式
}
