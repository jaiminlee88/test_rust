
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter { // 不用self，调用时用::
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;// 迭代器会产生 u32s

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();
    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}

#[test]
fn using_other_iterator_trait_methods() {
    let c = Counter::new().skip(1); // 2 3 4 5
    println!("=========={:?}", Counter::new().skip(6).next());
    println!("=========={:?}", Counter::new().skip(5).next());
    println!("=========={:?}", Counter::new().skip(2).next());

    let sum : u32 = Counter::new().zip(Counter::new().skip(1)) // skip(1) 丢掉第一项，zip并行迭代两个迭代器
                                  .map(|(a, b)| { a * b }) // 将两个迭代器的结果相乘
                                  .filter(|x| { x % 3 == 0}) // 过滤掉不能被3整除的数
                                  .sum(); // 求和
    assert_eq!(18, sum);
}