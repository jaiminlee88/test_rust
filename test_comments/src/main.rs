//! # Comments 注意这么写没有用，//！针对的是模块

use test_comments::add_one;
use test_comments::art::PrimaryColor;
use test_comments::art::SecondaryColor;
use test_comments::art::mix;

fn main() {
    println!("Hello, world!");
    println!("add_one(1) = {}", add_one(1));

    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    let orange = mix(red, yellow);
    println!("orange = {:?}", orange);
}
