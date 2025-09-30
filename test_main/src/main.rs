use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn test4() {
    println!("============test4=======");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("secret_number={}", secret_number);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line.");
        // let guess:i32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() { // trim() 去掉字符串两端的空白字符,parse() 将字符串转换为数字, 上面guess是String类型，这里guess是u32类型，允许复用
            Ok(num) => num, // parse成功，返回一个u32类型的值
            Err(_) => continue, // parse失败，继续循环
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
fn test3() {
    println!("==========test3===========");
    // --snip--
    // println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);

    let guess: u32 = guess.trim().parse().expect("Please type a number!");//遮蔽 （shadow） guess 之前的值
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }

}
fn test2() {
    println!("=======test2 start=======");
    // println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
    println!("Please input your guess.");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}


fn test1() {
    println!("======test1 start======");
    let x = 5;
    let y = 10;
    println!("x = {} and y = {}", x, y);
}

fn test0(){
    println!("=======test0 start=====");
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
fn main() {
    // test0();
    // test1();
    // test2();
    // test3();
    test4();
}
