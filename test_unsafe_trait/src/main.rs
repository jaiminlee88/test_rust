
unsafe trait Foo {

}

unsafe impl Foo for i32 {

}

fn test1() {
    println!("===========test1===========");
}

fn main() {
    test1();
}
