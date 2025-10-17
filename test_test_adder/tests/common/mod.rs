use test_test_adder;

pub fn setup() {
    println!("common::setup===> setup============"); // 我们希望这个函数能被多个测试文件的测试函数调用, 如果以单独文件放在上层目录，会默认为一个独立的crate测试
}