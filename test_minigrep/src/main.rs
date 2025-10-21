use std::env; //使用它的 args 函数来获取命令行参数

// cargo run searchstring example-filename.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
}
