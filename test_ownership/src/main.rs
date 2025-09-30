

fn test1() {
    println!("test1================");
    let s = "hello"; // s的scope在test1函数结束时结束
    println!("{}", s);
}

fn test2() {
    println!("test2================");
    let s = String::from("hello"); // s的scope在test2函数结束时结束
    println!("{}", s);

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    {
        // Rust 永远也不会自动创建数据的 “深拷贝”
        let s1 = String::from("hello");
        let s2 = s1; // 不会拷贝，所有权转移
        // println!("{}", s1); // s1的值已经被移动到s2，所以这里会报错
        println!("{}", s2);
    }

    {
        // 克隆
        let x = 5;
        let y = x; // 整数是已知且固定的，因此拷贝会更快
        println!("{}", x);
        println!("{}", y);

        let s1 = String::from("hello");
        let s2 = s1.clone(); // 主动调用clone方法，会创建一个深拷贝
        println!("{}", s1);
        println!("{}", s2);
    }
}

fn takes_ownership(s: String) {
    println!("[takes_ownership] {}", s);
}

fn makes_copy(x: i32) {
    println!("[makes_copy] {}", x);
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s // 返回s的所有权
}

fn takes_and_gives_back(s: String) -> String {
    println!("[takes_and_gives_back] {}", s);
    s // 返回s的所有权
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn test3() {
    println!("test3================");
    let s = String::from("hello");
    takes_ownership(s); // s的值被移动到takes_ownership函数
    // println!("{}", s); // s的值已经被移动到takes_ownership函数，所以这里会报错
    let x = 5;
    makes_copy(x); // x本身是可拷贝的，i32 char bool 等都是可拷贝的
    println!("{}", x); // x的值被拷贝到makes_copy函数，可以打印

    {
        // 变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它
        let s1 = gives_ownership();
        println!("{}", s1);

        let s2 = String::from("hello");
        let s3 = takes_and_gives_back(s2);
        // println!("{}", s2); // s2的值已经被移动到takes_and_gives_back函数，所以这里会报错
        println!("{}", s3);
    }

    {
        let s = String::from("hello");
        let (s1, length) = calculate_length(s);
        println!("{}", s1); // s的所有权已经被移动到calculate_length函数，不能再用，用元组一并返回的
        println!("{}", length);
    }
}

fn calculate_length_ref(s: &String) -> usize { // s就是一个结构体，里面有个ptr
    //函数签名使用  &  来表明参数  s  的类型是一个引用
    s.len()
}

fn change(s: &mut String) { // 函数签名使用  &mut  来表明参数  s  的类型是一个可变引用
    s.push_str(", world!");
}

fn test4() {
    println!("test4================");
    {
        let s1 = String::from("hello");
        let length = calculate_length_ref(&s1); // &s1  语法让我们创建一个 指向 值  s1  的引用，
        println!("{}", length); // s的所有权没有转移，所以可以继续使用
        println!("{}", s1);
    }


    {
        let mut s = String::from("hello");
        change(&mut s);
        println!("{}", s);
    }

    {
        let mut s = String::from("hello");
        let r1 = &mut s;
        // let r2 = &mut s; // 同一时间只能有一个可变引用
        println!("{}", r1);
        // println!("{}", r2);
    }

    {
        let mut s = String::from("hello");
        {
            let r1 = &mut s; // 这样是合法的，在一个域内
            println!("{}", r1);
        }
        let r2 = &mut s; // 同一时间只能有一个可变引用
        println!("{}", r2);
    }

    {
        let mut s = String::from("hello");
        let r1 = &s; // ok，引用是不可变的
        let r2 = &s; // ok
        println!("r1: {}", r1);
        println!("r2: {}", r2);
        let r3 = &mut s; // 同一时间只能有一个可变引用
        // println!("r1: {}, r2: {}, r3: {}", r1, r2, r3); // 这里会报错，因为r1和r2是不可变的，r3是可变的
        println!("r3: {}", r3); // 不在使用r1和r2后，可以再次使用r3
    }
}

// fn dangle() -> &String { // 返回引用，悬垂了，直接不能通过编译
//     let s = String::from("hello");// s 是一个新字符串 生命周期在函数结束时结束
//     &s // 返回引用，悬垂了
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}

fn test5() {
    println!("test5================");
    // let reference_to_nothing = dangle(); // 返回引用，悬垂了
    // println!("{}", reference_to_nothing);
    let reference_to_nothing = no_dangle();
    println!("{}", reference_to_nothing);
}

fn first_word(s :&String) -> usize {
    let bytes = s.as_bytes(); //转化为字节数组
    for (i, &item) in bytes.iter().enumerate() { // iter返回一个迭代器，可以遍历每个元素
        // enumerate  返回的元组包含索引和引用，&item 解引用
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn test6() {
    println!("test6================");
    let s = String::from("hello world");
    let word = first_word(&s);
    println!("{}", word);

    let hello = &s[0..5];
    let hello1 = &s[..5];
    let world = &s[6..11];
    println!("{}", hello);
    println!("{}", hello1);
    println!("{}", world);
}

fn main() {
    test1();
    test2(); // string
    test3(); // 所有权与函数
    test4(); // 引用与借用
    test5(); // 悬垂引用 dangling reference
    test6(); // 切片 slice
}
