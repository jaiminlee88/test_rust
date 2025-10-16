

fn find_largest(list: &Vec<i32>) -> i32 { // 只能接收 Vec
    32
}
fn find_largest_2(list: &[i32]) -> i32 { //&[i32] 可接收 Vec、数组、切片
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
fn test1() {
    println!("===test1==================");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = find_largest(&number_list);
    println!("{:?} The largest number is {}", number_list, result);
}


fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }   
    }
    largest
}
fn test2() {
    println!("===test2==================");
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("{:?} The largest number is {}", number_list, result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("{:?} The largest char is {}", char_list, result);
}

struct Point<T, U> {
    x: T,
    y: U,
}

fn test3() {
    println!("===test3==================");
    let point = Point { x: 1, y: 2.0 };
    println!("point: {} {}", point.x, point.y);
    let point = Point { x: 1.5, y: 2.5 };
    println!("point: {} {}", point.x, point.y);
}

#[derive(Debug)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn test4() {
    println!("===test4==================");
    let x: Option<i32> = Some(5);
    let y: Option<f64> = Some(5.0);
    println!("x: {:?}, y: {:?}", x, y);
    let x: Result<i32, String> = Result::Ok(5); // 必须使用Result::，否则编译器会以为使用标准库版本
    let y: Result<i32, String> = Result::Err("error".to_string());
    println!("x: {:?}, y: {:?}", x, y);
}


struct Point1<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point1<T,U> {
    fn x(&self) -> &T {
        &self.x
    }   
    fn y(&self) -> &U {
        &self.y
    }

    fn mixup<V,W>(self, other: Point1<V,W>) -> Point1<T,W> { // 使用self 而不是 &self 是因为我们想要获取所有权
        Point1 {
            x: self.x, // 出现了所有权转移
            y: other.y,
        }
    }
}

fn test5() {
    println!("===test5==================");
    let point1 = Point1 { x: 1, y: 2.0 };
    println!("point1: {} {}", point1.x(), point1.y());
    let point2 = Point1 { x: "Hello", y: 'c' };
    let point3 = point1.mixup(point2);
    println!("point3: {} {}", point3.x(), point3.y());
    // println!("point1: {} {}", point1.x(), point1.y()); // 所有权转移后，point1 无法再使用
}

// 定义指定了签名中所有的引用必须有相同的生命周期 'a
fn longest<'a>(x: &'a str, y:&'a str) -> &'a str { // // 带有显式生命周期的引用 'a
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test6() {
    println!("===test6==================");
    let string1 = String::from("abcd");
    {
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    let string1 = String::from("abcd");
    let result;
    {
        let string2 = "xyz"; // 字符串字面值（&'static str）,生命周期是'static,程序整个运行期间都有效
        result = longest(string1.as_str(), string2);
    }
    println!("The longest string is {}", result);

    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz"); // 声明周期不是static，跳出作用域后，string2 被释放，result 无法使用
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}
fn test7() {
    println!("===test7==================");
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { part: first_sentence }; // 传入引用了
    println!("i.part-> {}", i.part); 
    let part = i.announce_and_return_part("Plllaase!!!!");
    println!("part-> {}", part);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn test8() {
    println!("===test8==================");
    let string1 = String::from("hello world");
    let result = first_word(&string1);
    println!("The first word is: {}", result);
}

fn test9() {
    println!("===test9==================");
    let s:'static str = "I have a static lifetime.";
}
fn main() {
    test1(); // find largest number in a vector
    test2(); // 函数泛型
    test3(); // 结构体泛型
    test4(); // 枚举泛型
    test5(); // 方法泛型
    test6(); // lifetime
    test7(); // lifetime
    test8(); // lifetime elision
    test9(); // lifetime static lifetime
}