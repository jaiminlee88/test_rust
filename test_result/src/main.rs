
use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::io::Read;

fn test1() {
    println!("===test1==================");
    let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => panic!("Problem opening the file: {:?}", e),
    // };
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            _ => panic!("Problem opening the file: {:?}", e),
        }
    };
}

fn test2() {
    println!("===test2==================");
    let f = File::open("hello1.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello1.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello2.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e), // return Err(e) is the same as panic!("Problem opening the file: {:?}", e)
    };
    let mut s = String::new();
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(e) => Err(e),
    // }
    f.read_to_string(&mut s)?; // 简写,使用  ?  运算符向调用者返回错误的函数,与 match 运行一致
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}
fn test3() {
    println!("===test3==================");
    let username = read_username_from_file();
    println!("username: {:?}", username);
}

fn main() {
    test1();
    test2();
    test3();
}
