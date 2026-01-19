use test_super_traits::{Counter, MyIterator, Iterator};
use std::ops::Add;
use std::fmt;

fn test1() {
    println!("==================test1==================");
    let mut c = Counter::new();

    let v1 : Option<i32> = MyIterator::<i32>::next(&mut c);
    println!("v1: {:?}", v1);

    let v2 : Option<i32> = Iterator::next(&mut c);
    println!("v2: {:?}", v2);
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point; // 决定返回类型
    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

fn test2() {
    println!("==================test2==================");
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2;
    println!("p3: {:?}", p3);
}

#[derive(Debug)]
struct Millimeters(u32);
#[derive(Debug)]
struct Meters(u32);

/*
trait Add<RHS=Self> { // 默认参数是Self
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
*/
impl Add<Meters> for Millimeters {
    type Output = Millimeters;
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + other.0 * 1000)
    }
}

fn test3() {
    println!("==================test3==================");
    let one_hundred = Millimeters(100);
    let one_meter = Meters(1);
    let sum = one_hundred + one_meter;
    println!("sum: {:?}", sum);
}

trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}
struct Human;
impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying");
    }
}
impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying");
    }
}
impl Human {
    fn fly(&self) {
        println!("Human flying");
    }
}

fn test4() {
    println!("==================test4==================");
    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
}


trait Animal {
    fn baby_name() -> String;
}
struct Dog;
impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}
fn test5() {
    println!("==================test5==================");
    println!("dog baby name: {}", Dog::baby_name());
    println!("dog baby name: {}", <Dog as Animal>::baby_name());
}


trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", output);
        println!("{}", "*".repeat(len + 4));
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl OutlinePrint for Point {}
fn test6() {
    println!("==================test6==================");
    let p = Point { x: 1, y: 2 };
    p.outline_print(); // 
}


struct Wrapper(Vec<String>);
impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
fn test7() {
    println!("==================test7==================");
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w: {}", w);
}
fn main() {
    test1(); // 关联类型
    test2(); // 运算符 trait
    test3(); // 运算符 trait
    test4(); // 完全限定语法与消歧义：调用相同名称的方法
    test5(); // <Type as Trait>::function(receiver_if_method, next_arg, ...);
    test6(); // 父 trait 用于在另一个 trait 中使用某 trait 的功能
    test7(); // newtype 模式
}
