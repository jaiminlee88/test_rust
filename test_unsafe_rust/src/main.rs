
fn test1() {
    println!("===========test1===========");
    let mut num = 5; // 当前栈上创建一个i32
    let r1 = &num as *const i32; // 创建一个不可变引用&i32，并转换为不可变裸指针*const i32
    let r2 = &mut num as *mut i32; // 创建一个可变引用&mut i32，并转换为可变裸指针*mut i32
    println!("r1 = {:p}, r2 = {:p}", r1, r2);
    unsafe {
        println!("r1 = {}, r2 = {}", *r1, *r2); // 使用裸指针解引用
    }
}

unsafe fn dangerous() {
    println!("===========dangerous===========");
    println!("Hello, dangerous world!");
}
fn test2() {
    println!("===========test2===========");
    unsafe {
        dangerous();
    }
}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr(); // *mut i32
    assert!(mid <= len);
    unsafe {
        (std::slice::from_raw_parts_mut(ptr, mid)
        , std::slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}
fn test3() {
    println!("===========test3===========");
    let mut v = vec![1, 2, 3, 4, 5];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    println!("a = {:?}, b = {:?}", a, b);

    let (a, b) = split_at_mut(r, 4);
    println!("a = {:?}, b = {:?}", a, b);
}


use libc::c_int;
unsafe extern "C" {
    fn abs(input: i32) -> i32;
}
fn test4() {
    println!("===========test4===========");
    unsafe {
        println!("abs(-3) = {}", abs(-3));
    }
}

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}
fn test5() {
    println!("===========test5===========");
    println!("HELLO_WORLD = {}", HELLO_WORLD);

    add_to_count(3);
    unsafe {
        let val = COUNTER;
        println!("COUNTER = {}", val);
    }
}

fn main() {
    test1(); // reference raw pointer
    test2(); // unsafe function
    test3(); // unsafe codes' safe abstraction
    test4(); // extern "C" function，使用C函数
    test5(); // static variable
}
