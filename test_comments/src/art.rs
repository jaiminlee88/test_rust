//! # Art
//!
//! A library for modeling artistic concepts.

// 使用pub use会后，可以不加kind，main中直接use art::PrimaryColor来使用PrimaryColor枚举
pub use kinds::PrimaryColor; // 导出公共枚举，即art模块中的PrimaryColor和SecondaryColor可以被其他模块使用
pub use kinds::SecondaryColor; // 使用者可以直接use art::SecondaryColor来使用SecondaryColor枚举
pub use utils::mix; // 导出公共函数，即art模块中的mix函数可以被其他模块使用，使用者可以直接use utils::mix来使用mix函数

pub mod kinds {
    /// The primary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// The secondary colors according to the RYB color model.
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create a secondary color.
    /// 这部分内容会反应到doc中
    ///
    /// # Examples
    ///
    /// ```
    /// use test_comments::art::kinds::PrimaryColor;
    /// use test_comments::art::kinds::SecondaryColor;
    /// use test_comments::art::utils::mix;
    ///
    /// let red = PrimaryColor::Red;
    /// let yellow = PrimaryColor::Yellow;
    /// let orange = mix(red, yellow);
    /// assert_eq!(orange, SecondaryColor::Orange);
    /// ```
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            _ => unreachable!(), // 需要保证永远不可能到达这里
            // _ => panic!("不合理的颜色组合: {:?} 和 {:?}", c1, c2), // 出发panic
        }
    }
}