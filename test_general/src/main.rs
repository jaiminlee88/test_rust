use std::io;

fn test0() {
    println!("=========test0==========");
    let x = 5;
    println!("x is {}", x);
    // x = 6;
    // println!("x is overwrite to {}", x);

    let mut y = 6;
    println!("mut y is {}", y);
    y = 7;
    println!("mut y is overwrite to {}", y);
}

fn test1() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("const THREE_HOURS_IN_SECONDS is {}", THREE_HOURS_IN_SECONDS);
}

fn test2 () {
    println!("==========test2=====shadow===");
    let x = 5;
    println!{"[1] x={}",x};
    let x = x + 1; // not mut, let is used to create new instance
    println!{"[2] x={}",x};
    {
        let x = x * 2; // different from previous x
        println!{"[3] x={}",x};
        println!("The value of x in the inner scope is: {}", x);
    }
    println!{"[4] x={}",x};
    println!("The value of x is: {}", x);


    let spaces = "   ";
    println!{"[5] spaces={}",spaces};
    let spaces = spaces.len();
    println!{"[6] spaces={}",spaces};
    // not allowed
    // let mut spaces = "   ";
    // spaces = spaces.len();

    let _t = true;
    let _f: bool = false; // with explicit type annotation

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    println!("char c={}", c);
    println!("char z={}", z);
    println!("char heart_eyed_cat={}", heart_eyed_cat);
    
}

fn test3() {
    println!("===========test3==dtypes===");
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess={}",guess);
    // 8 位	    i8	u8
    // 16 位	i16	u16
    // 32 位	i32	u32
    // 64 位	i64	u64
    // 128 位	i128	u128
    // arch	    isize	usize
    // let num8i: i8 = 257; // overflow
    let num8i: i8 = 126;
    println!("num8i={}", num8i);

    // 十进制	98_222 _为分隔符
    // 十六进制	0xff
    // 八进制	0o77
    // 二进制	0b1111_0000
    // 字节 (仅限于 u8)	b'A'
    let num8u: u8 = b'A';
    println!("num8u={}", num8u);


    let numf64 = 2.5; // f64
    let numf32: f32 = 3.5; // f32
    println!("numf64={}", numf64);
    println!("numf32={}", numf32);
}

fn test4() {
    println!("=========test4==cal====");
    // addition
    let sum = 5 + 10;
    println!("[1] sum={}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("[2] difference={}",difference);
    // multiplication
    let product = 4 * 30;
    println!("[3] product={}",product);
    // division
    let quotient = 56.7 / 32.2;
    println!("[4] quotient={}",quotient);
    let floored = 2 / 3; // Results in 0
    println!("[5] floored={}",floored);
    let floored_float = 2.0 / 3.0; // float
    println!("[6] floored_float={}",floored_float);
    // remainder
    let remainder = 43 % 5;
    println!("[7] remainder={}",remainder);
}

fn test5() {
    println!("===========test5====compound===");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tuple tup[0]={}, tup[1]={}, tup[2]={}",tup.0, tup.1, tup.2);

    let tup = (600, 8.4, 5);
    let (x, y, z) = tup;
    println!("The value of y is: {} {} {}", x, y, z);

    let arr = [1,2,3,4];
    println!("array: arr[0]={} arr[1]={} arr[2]={} arr[3]={}", arr[0], arr[1], arr[2], arr[3]);

    let arr1: [i32; 5] = [1, 2, 3, 4, 5];
    println!("array: arr1[0]={} arr1[1]={} arr1[2]={} arr1[3]={}", arr1[0], arr1[1], arr1[2], arr1[3]);

    let arr2 = [3; 5];
    println!("array: arr2[0]={} arr2[1]={} arr2[2]={} arr2[3]={}", arr2[0], arr2[1], arr2[2], arr2[3]);

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    println!("months[0]={} months[3]={}", months[0], months[3]);

}

fn test6() {
    println!("==============test6==============");
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index, element
    );
}


fn test7() {
    println!("==========test7========");
    another_function();
    // another_function(4);
    another_function_i32(4);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function()");
}
// overload not supported
// fn another_function(x: i32) {
//     println!("another_function(x: i32) The value of x is: {}", x);
// }
// alternative
fn another_function_i32(x: i32) {
    println!("another_function_i32(x: i32) The value of x is: {}", x);
}
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

fn test8 () {
    println!("===============test8=======");
    let y = { // expression 表达式，返回值
        let x = 3; // statement 语句，不返回值
        x + 1 // 无分号
    };
    println!("The value of y is: {}", y);
    println!("The value of y + 5 is: {}", add_five(y));
}
fn add_five(x: i32)->i32 {
    x + 5 // 无分号
}

fn test9() {
    println!("=========test5 control flows=========");
    let number = 3;
    if number < 5 { // condition must be boolean
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // let condition = 5; // invalid, must be boolean
    let condition = true;
    let number = if condition { 5 } else { 6 };
    // let number = if condition { 5 } else { "abd" }; // invalid
    println!("The value of number is: {}", number);
}

fn test10() {
    println!("===============test10====loop===");
    const MAX_NUM_CNT: usize = 5;
    let arr: [i32; MAX_NUM_CNT] = [1,2,3,4,5];
    let mut index = 0;
    loop {
        if index >= MAX_NUM_CNT {
            break;
        }
        println!("loop arr[{}]={}",index, arr[index]);
        index += 1;
    }
}

fn test11() {
    println!("===============test11====loop1===");
    let mut count = 0;
    'counting_up: loop { //counting_up 为loop label 标签
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    let mut counter = 0;
    let result = loop {
        counter += 1; // 访问外部
        println!("The counter={}", counter);
        if counter == 3 {
            break counter * 2; // expression
        }
    };
    println!("The result is {}", result);
}

fn test12() {
    println!("===========test12==while and for====");
    let mut number = 3;
    while number != 0 {
        println!("while number={}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("while the value is: {}", a[index]);
        index += 1;
    }

    for element in a {
        println!("for the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("rev {}!", number);
    }
    println!("LIFTOFF!!!");
}

fn Fibonacci(n : i32) -> i32 {
    if n < 0 {
        return 0;
    } else if n <= 1 {
        return n;
    } else {
        return Fibonacci(n -1) + Fibonacci(n - 2);
    }
}
fn test13() {
    println!("============test13==Fibonacci===");
    let n = 5;
    let mut index = 0;
    while index <= n {
        println!("Fibonacci({})={}", index, Fibonacci(index));
        index += 1;
    }
}
fn main() {
    // println!("Hello, world!");
    test0();
    test1();
    test2();
    test3();
    test4();
    test5();
    // test6();
    test7();
    test8();
    test9();
    test10(); // loop
    test11(); // loop
    test12(); // while
    test13(); // Fibonacci
}
