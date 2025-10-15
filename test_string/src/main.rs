

fn test1() {
    println!("===test1==================");
    {
        let mut s = String::new();
    }

    {
        let data = "initial contents"; // 字符串是 UTF-8 编码的
        let s = data.to_string();
        println!("{}", s);

        let s = "initial contents".to_string(); // 使用  to_string  方法从字符串字面值创建  String
        println!("{}", s);

        let s = String::from("initial contents"); // 使用  from 方法从字符串字面值创建  String
        println!("{}", s);
    }

    {
        let mut s = String::from("hello");
        s.push_str("bar"); // 使用 push_str 方法向 String 附加字符串 slice
        println!("{}", s);

        let s2 = "bar";
        s.push_str(s2);
        println!("{}", s);
        println!("{}", s2);

        s.push('l');
        println!("{}", s);
    }

    {
        let s1 = String::from("hello, ");
        let s2 = String::from("world");
        let s3 = s1 + &s2; // 使用 + 运算符连接字符串,使用第二个字符串的 引用 与第一个字符串相加
        // let s4 = s3 + s2; // 不能两个string相加
        println!("{}", s3);
        // println!("{}", s1); // 这里 s1 的所有权被转移给了 s3，所以 s1 不能再使用
        println!("{}", s2); // 这里 s2 的所有权没有被转移，所以 s2 可以继续使用

        let s1 = String::from("tic");
        let s2 = String::from("tac");
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = s1 + "-" + &s2 + "-" + &s3; // 笨拙
        println!("{}", s);
    }

    {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{}-{}-{}", s1, s2, s3); // 使用 format! 宏连接字符串
        println!("{}", s);
        println!("{}", s1); // 这里 s1 的所有权没有被转移，所以 s1 可以继续使用
    }

}

fn test2() {
    println!("===test2==================");
    {
        let s = String::from("hello");
        // let hello = &s[0]; // invalid utf-8
        let hello = &s[0..5];
        println!("{}", hello);
    }

    {
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        // let s = &hello[0..1]; // panic, 因为 1 不是有效的 UTF-8 字符边界
        println!("{}", s);
    }
}

fn test3() {
    println!("===test3==================");
    {
        for c in "Здравствуйте".chars() {
            print!("{} ", c);
        }
        println!("");
        for b in "Здравствуйте".bytes() {
            print!("{} ", b);
        }
        println!("");
    }
}

fn main() {
    test1(); // 拼接
    test2(); // 索引
    test3(); // 遍历
}
