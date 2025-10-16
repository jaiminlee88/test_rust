

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

fn main() {
    test1(); // find largest number in a vector
    test2(); // 函数泛型
    test3(); // 结构体泛型
    test4(); // 枚举泛型
    test5(); // 方法泛型
}
