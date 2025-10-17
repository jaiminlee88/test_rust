/*
我们需要在文件顶部添加 use test_test_adder 。 这是因为每一个 tests 目录中的测
试文件都是完全独立的 crate， 所以需要在每一个文件中导入库
*/
use test_test_adder;
mod common;

/*
不需要告诉rust #[cfg(test)] ，
*/
#[test]
fn integration_test_it_adds_two() {
    common::setup();
    println!("it_adds_two===> it_adds_two============");

    assert_eq!(4, test_test_adder::add_two(2));
}

#[test]
fn integration_test_it_adds_two_2() {
    println!("it_adds_two===> it_adds_two============");
    assert_eq!(8, test_test_adder::add_two(6));
}

// cargo test --test integration_test