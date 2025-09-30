


fn test1() {
    println!("test1 start==================");
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS={}", MAX_POINTS);
    let mut x = 5;
    println!("x={}", x);
    x = 6;
    println!("x={}", x);
}

fn test2() {
    println!("test2 start==================");
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x={}", x); // 这个x是内层块的x，不是外层块的x
    }
    println!("x={}", x);
}

fn test3() {
    println!("test3 start==================");
    let space = "    ";
    let space = space.len();
    println!("space={}", space);
    // space = space.len(); // 这里space是usize类型，不是str类型，所以不能赋值
}

fn test4() {
    println!("test4 start==================");
    let guess : u32 = "42".parse().expect("Not a number");
    println!("guess={}", guess);
}

fn test5() {
    println!("test5 start==================");
    let x = 2.5; // f64
    let y: f32 = 3.0; // f32
    println!("x={}, y={}", x, y);
}

fn test6() {
    println!("test6 start==================");
    let t: bool = true;
    println!("t={}", t);
}

fn test7() {
    println!("test7 start==================");
    let c = 'z'; // char, 4个字节
    println!("c={}", c);
}

fn test8() {
    println!("test8 start==================");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("tup={:?}", tup);
    let (x, y, z) = tup;
    println!("x={}, y={}, z={}", x, y, z);
    let x = tup.0; // 通过索引解构，访问tuple中的元素
    let y = tup.1;
    let z = tup.2;
    println!("x={}, y={}, z={}", x, y, z);
}

fn test9() {
    println!("test9 start==================");
    let a = [1, 2, 3, 4, 5]; // 数组，长度固定，类型相同，在栈上分配
    println!("a={:?}", a);
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // 数组，长度固定，类型相同，在栈上分配
    println!("a={:?}", a);
    let a = [3; 5]; // 数组，长度固定，类型相同，在栈上分配，初始值为3，5个元素
    println!("a={:?}", a);
    let a = [1, 2, 3, 4, 5];
    println!("a[0]={}", a[0]);
}

fn main() {
    test1(); // 变量
    test2(); // 隐藏 shadowing
    test3(); // 数据类型
    test4(); // int
    test5(); // float
    test6(); // bool
    test7(); // char
    test8(); // tuple
    test9(); // array
}
