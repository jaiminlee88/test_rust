pub struct Counter {
    count : i32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }
}

// 下面是泛型
// 一个trait可以有多个泛型，维护很不容易，但是由使用者决定是什么
pub trait MyIterator<T> {
    fn next(&mut self) -> Option<T>;
}
impl MyIterator<i32> for Counter {
    fn next(&mut self) -> Option<i32> {
        self.count += 1;
        Some(self.count)
    }
}


/*
关联类型用法 associate types，适用场景
Iterator
运算符 trait（Add / Mul）
容器“元素类型”
I/O（Read / Write 的 buffer 类型）
协议 / 状态机里的“下一状态”
*/
pub trait Iterator {
    type Item; // Item 不是泛型,trait 本身也 不知道 Item 是什么
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = i32; // 显式指定 Item 的类型，后序就不能再改了，开发者自己决定是什么
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

//