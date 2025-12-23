use std::env; //使用它的 args 函数来获取命令行参数

use std::process;


use test_minigrep::Config;

// cargo run searchstring example-filename.txt
fn main() {
    // let args: Vec<String> = env::args().collect();
    // // println!("{:?}", args);

    // let config: Config = parse_config(&args);
    // let config = Config::new(&args).unwrap_or_else(|err| { // 如果错误会把错误传给两个竖线的err
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });
    // // println!("action: {}", config.action);
    // // println!("filename: {}", config.filename); // 标准输出

    let config = Config::new(env::args()).unwrap_or_else(|err| { // 使用迭代器传递参数
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // let contents = fs::read_to_string(filename)
    //     .expect("Should have been able to read the file");
    // println!("File contents:\n{}", contents);
    if let Err(e) = test_minigrep::run(config) { // run不返回对象，只返回Result，所以用if let Err(e) = run(config)来处理错误
        eprintln!("Application error: {}", e); // 标准错误输出
        process::exit(1);
    }
}