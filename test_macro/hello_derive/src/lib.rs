// ===========过程宏===========
/*
过程宏是 Rust 程序：它接收一段语法树，返回另一段语法树。
  - 过程宏 = 在编译期运行的 Rust 函数，用来“读 AST、改 AST、再吐回 AST”。
过程宏可以分为以下几种：
1. 派生宏（Derive Macros）：为类型实现特定的 trait。
2. 属性宏（Attribute Macros）：为类型或函数添加特定的属性。
3. 函数宏（Function Macros）：为函数添加特定的属性。

*/
extern crate proc_macro;
use proc_macro::TokenStream; // TokenStream 是 Rust 代码的“语法 token 表示”
// ===========派生宏===========
#[proc_macro_derive(Hello)] /// 定义一个派生宏，宏名叫 Hello。 当看到 #[derive(Hello)] 时，rust 编译器会调用 hello_derive 函数
pub fn hello_derive(_input: TokenStream) -> TokenStream {
    // 这里的 input 是：struct User; 的 token 表示
    // 现在先不解析，直接演示机制

    let expanded = r#"
        impl User {
            pub fn hello() {
                println!("Hello from User");
            }
        }
    "#;

    expanded.parse().unwrap() // 直接返回一段 合法 Rust 代码,编译器会插入到代码中，再继续编译
}