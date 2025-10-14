
fn test1() {
    println!("===test1===============");
    let v1 = vec![1,2,3]; // vec!是宏，会自动推导出i32
    for elem in v1 {
        print!("{} ", elem);
    }
    println!();
}

fn test2() {
    println!("===test2==============");
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    for elem in v {
        print!("{} ", elem);
    }
    println!();
}

fn test3() {
    println!("===test3==============");
    let v = vec![1,2,3,4,5];
    let third: &i32 = &v[2];
    println!("The third elem is {}", third);

    match v.get(2) {
        Some(third) => println!("The third elem is {}", third),
        None => println!("There is no third element.")
    }

    // println!("{} ", &v[100]); // 出错
    let does_not_exist = v.get(100);
    if does_not_exist.is_none() {
        println!("does_not_exist.")
    } else {
        println!("does_exist {:?} ", does_not_exist);
        println!("does_exist {} ", does_not_exist.unwrap());
    }

    if let Some(x) = does_not_exist {
        println!("exists: {}", x);
    } else {
        println!("does not exists.")
    }
}

fn test4() {
    println!("===test4===================");
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    // v.push(0); // 有可能触发重新分配，first会变得无效，所以编译不过
    println!("first is {}", first);
}

fn test5() {
    println!("===test5===================");
    let mut v = vec![100,32,57];
    for i in &v {
        println!("{}",i);
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{}",i);
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}
// 提供打印函数
// use std::fmt;
// impl fmt::Display for SpreadsheetCell {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             SpreadsheetCell::Int(i) => write!(f, "{}", i),
//             SpreadsheetCell::Float(x) => write!(f, "{}", x),
//             SpreadsheetCell::Text(s) => write!(f, "{}", s),
//         }
//     }
// }
fn test6() {
    println!("===test6==================");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12)
    ];
    for v in row {
        print!("{:?} ", v);
    }
    println!("");
}

fn main() {
    test1();
    test2();
    test3();
    test4();
    test5();
    test6();
}
