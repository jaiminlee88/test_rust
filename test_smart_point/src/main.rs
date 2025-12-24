
enum List {
    Cons(i32, Box<List>), // Cons存储一个值+一个Box<List>
    Nil,
}

use crate::List::{Cons, Nil};
use std::ops::Deref;

fn test1() {
    println!("====================test1====================");
    let b = Box::new(1); // 堆上分配一个i32，b是一个智能指针，指向堆上的i32
    println!("b: {}", b);
}

fn test2() {
    println!("====================test2====================");
    let _list = Cons(1, 
            Box::new(Cons(2, 
                Box::new(Cons(3, 
                    Box::new(Nil)))))); // Cons存储一个值+一个Cons
}

fn test3() {
    println!("====================test3====================");
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    assert_eq!(5, *y);
}

struct MyBox<T>(T); // 元组结构体，后面跟着一个tuple
// struct MyBox<T> {
//     0: T, // 概念上，与上面等价，Rust 语法不允许真的写 0
// }
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> { // 实现 Deref trait 来定义如何解引用,可以*y解引用，访问内部数据
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn test4() {
    println!("====================test4====================");
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let x = MyBox::new(5);
    let y : &i32 = &x; // 编译器自动展开为 let y : &i32 = x.deref();
    assert_eq!(5, *y); // *y == *(y.deref())
    println!("x: {}", x.deref());
    println!("y: {}", *y);
}

fn hello(name: &str) { // 参数是引用，但接受的是实现了 Deref trait 的类型
    println!("Hello, {}!", name);
}

fn test5() {
    println!("====================test5====================");
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // 通过  deref  调用将  &MyBox<String>  变为&String
}


struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer { // 退出时会自动调用drop方法
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn test6() {
    println!("====================test6====================");
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    let e = CustomSmartPointer { data: String::from("last stuff") };
    println!("CustomSmartPointers created.");
    // e.drop(); // 手动调用drop方法，不允许显式调用，会导致double free
    std::mem::drop(e); // 手动调用drop方法，允许显式调用，不会导致double free
    println!("CustomSmartPointers dropped before the end of main.");
}

enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
use ListRc::{Cons as RcCons, Nil as RcNil};
use std::rc::Rc; // 引用计数智能指针
fn test7() {
    println!("====================test7====================");
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = RcCons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

enum ListRefCell {
    Cons(i32, RefCell<Rc<ListRefCell>>),
    Nil,
}
use ListRefCell::{Cons as RefCellCons, Nil as RefCellNil};
use std::cell::RefCell;  // 获得内部可变性的方法
fn test9() {
    println!("====================test9====================");
    let a = Rc::new(RefCellCons(5, RefCell::new(Rc::new(RefCellNil))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RefCellCons(3, RefCell::new(Rc::clone(&a)));
    println!("count after creating b = {}", Rc::strong_count(&a));
}

fn test8() {
    println!("====================test8====================");
    let a = Rc::new(RcCons(5, Rc::new(RcCons(10, Rc::new(RcNil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = RcCons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
}

use std::rc::{Weak};
// use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
fn test10() {
    println!("====================test10====================");
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
    
    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // 将branch的弱引用赋值给leaf的父引用
        
        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );
        
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf), // branch->leaf
            Rc::weak_count(&leaf),
        );
    }
        
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
    );
}

fn main() {
    test1(); // box pointer
    test2(); // enum pointer
    test3(); // ref pointer
    test4(); // self defined box pointer
    test5(); // deref coercion
    test6(); // drop trait
    test7(); // shared pointer
    test8();
    test10(); // weak pointer
}
