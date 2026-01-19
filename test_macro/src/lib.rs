/*
Rust 的宏是 Rust 语言中的一种强大的工具，用于生成代码。它们允许你在编译时动态地生成代码，而不是在运行时。
宏有几种不同的类型：
1. 声明式宏（Declarative Macros）：使用 macro_rules! 定义，是最常见的宏类型。
2. 过程宏（Procedural Macros）：使用 proc_macro 定义，可以生成代码，也可以处理代码。
3. 派生宏（Derive Macros）：使用 derive 定义，可以为类型实现特定的 trait。
4. 属性宏（Attribute Macros）：使用 #[...] 定义，可以为类型或函数添加特定的属性。
5. 类属性宏（Attribute-like Macros）：使用 #![...] 定义，可以为类型或函数添加特定的属性。
*/

// ===========声明式宏===========
#[macro_export]// 把这个myvec宏导出到 crate 根，使得其他 crate 也能用
macro_rules! myvec { // 定义一个名为 vec 的声明式宏（不是函数）
    /* 宏的“匹配规则”
        - $x:expr
          - $x 是一个变量，匹配任意rust表达式
          - expr	表达式
          - stmt	语句
          - ident	标识符
          - ty	类型
          - pat	模式
          - block	代码块
        - $( ... ),*
          - 重复匹配语法，重复匹配语法中，* 表示重复匹配 0 次或多次，+ 表示重复匹配 1 次或多次，? 表示重复匹配 0 次或 1 次
          - 且以逗号分隔
            - vec![]
            - vec![1]
            - vec![1, 2]
            - vec![1, 2, 3]
        - => { ... } 展开模板
          - 展开模板中，... 表示展开模板，$x 表示匹配的变量，* 表示重复匹配 0 次或多次
    */
    ($($x:expr),*) => { // 多包这层，展开为一个表达式，如果没有这个，就只能当一个语句用了
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}


