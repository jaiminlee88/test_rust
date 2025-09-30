

fn test1() {
    println!("test1 start==================");
    let number = 6;
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // if number { // 非bool值不能直接判断
    //     println!("number was five");
    // }
}
fn test2() {
    println!("test2 start==================");
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3 or 2");
    }
    // let condition = true;
    // let number = if condition { 5 } else { 6 };
    // println!("number={}", number);
}   
fn test3() {
    println!("test3 start==================");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number={}", number);
}
fn test4() {
    println!("test4 start==================");
    let mut count = 0;
    loop {
        count += 1;
        if count == 10 {
            break;
        }
    }
    println!("count={}", count);

    count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count * 2; // break 可以返回值
        }
    };
    println!("result={}", result);
}

fn test5() {
    println!("test5 start==================");
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
}
fn test6() {
    println!("test6 start==================");
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    for element in a.iter() {
        println!("iter: the value is: {}", element);
    }
    for number in 1..4 {
        println!("range: the value is: {}", number);
    }
    for number in (1..4).rev() { // 倒序
        println!("{}!", number);
    }
}
fn test7() {
    println!("test7 start==================");
    let mut n = 2;
    let mut prev1 = 0;
    let mut prev2 = 1;
    while n > 0 {
        let next = prev1 + prev2;
        prev1 = prev2;
        prev2 = next;
        n -= 1;
    }
    println!("n={}, prev1={}, prev2={}, nth={}", n, prev1, prev2, prev2);
}
fn main() {
    test1(); // if
    test2(); // if else
    test3(); // let + if
    test4(); // loop
    test5(); // while
    test6(); // for
    test7(); // fabonacci
}
