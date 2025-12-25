//! # Comments
//! 这是用于介绍模块的

pub mod art;

/// 给一个数字加1
///
/// # Examples
///
/// ```
/// use test_comments::add_one; // 测试时相当于有个新的main函数，所以必须use
/// let i = 1;
/// assert_eq!(add_one(i), 2);
/// ```
pub fn add_one(i : i32) -> i32 {
    i + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(add_one(1), 2);
    }
}