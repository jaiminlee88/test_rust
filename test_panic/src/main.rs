fn test1() {
    println!("===test1==================");
    panic!("crash and burn");
}

fn test2() {
    println!("===test2==================");
    let v = vec![1, 2, 3];
    v[99];
}

fn main() {
    // test1();
    // test2(); // RUST_BACKTRACE=1 cargo run
    test3();
}
