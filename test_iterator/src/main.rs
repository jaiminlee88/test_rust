
fn test1() {
    println!("=========test1=========");
    let v = vec![1, 2, 3];

    for x in v.iter() { // &T 提供, 不可变引用，只能读不能写
        println!("{}", x);
    }

    println!("--------------------------------");
    let mut v = vec![4, 5, 6];
    for x in v.iter_mut() { // &mut T 提供, 可变引用，可以读写
        *x += 1; // 解引用，修改值
        println!("{}", x);
    }

    println!("--------------------------------");
    let v = vec![7, 8, 9];
    for x in v.into_iter() { // 按值迭代, v 不能再使用，已被 move,迭代获得 T 本身的所有权
        println!("{}", x); // 消耗vec
    }
    // println!("v: {:?}", v); //  v 不能再使用，已被 move
    // println!("{}", v.len()); //  v 不能再使用，已被 move

    println!("--------------------------------");
    let v = vec![10, 11, 12];
    for x in (&v).into_iter() { // 按引用迭代, v 可以继续使用，因为 &v 是借用，等价于v.iter()
        println!("{}", x); // 不消耗vec
    }
    println!("v: {:?}", v);

    println!("--------------------------------");
    let mut v = vec![13, 14, 15];
    for x in (&mut v).into_iter() { // 按引用迭代, v 可以继续使用，因为 &v 是借用，等价于v.iter_mut()
        *x += 1; // 解引用，修改值
        println!("{}", x); // 不消耗vec
    }
    println!("v: {:?}", v);

}

fn test2() {
    println!("=========test2=========");
    let v = vec![1, 2, 3];
    let mut iter = v.iter();
    let total : i32 = iter.sum();
    println!("total: {}", total);
    // println!("iter: {:?}", iter); // 迭代器已被消耗，不能再使用
    println!("v: {:?}", v); // v 可以继续使用，不可变引用，不消耗vec
}

fn test3() {
    println!("=========test3=========");
    let v1 : Vec<i32> = vec![1, 2, 3];
    println!("v1: {:?}", v1);
    v1.iter().map(|x| {x + 1}); // 什么也没做，lazy evaluation，惰性求值，必须调用方法
    println!("v1: {:?}", v1);
    let v2 : Vec<_> = v1.iter().map(|x| {x + 1}).collect(); // 使用collect消耗迭代器，获得新的vec
    println!("v1: {:?}", v1); // v1 可以继续使用，不可变引用，不消耗vec
    println!("v2: {:?}", v2); // v2 是新的vec，是v1.iter().map(|x| {x + 1}).collect()的结果
}

fn main() {
    println!("Hello, world!");
    test1(); // 迭代器
    test2(); // 消耗迭代器
    test3(); // 迭代器适配器（iterator adaptors）
}
