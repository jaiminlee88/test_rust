use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub action: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // pub fn new(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("not enough arguments");
    //     }
    //     let action = args[1].clone();
    //     let filename = args[2].clone();
    //     //  is_err  会返回 false 并将进行大小写不敏感搜索
    //     let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 如果环境变量CASE_INSENSITIVE不存在，则认为是敏感的
    //     Ok(Config { action, filename, case_sensitive })
    // }

    // 使用迭代器传递参数
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next(); // 丢掉第一个参数，第一个参数是程序名

        let action = match args.next() { // 迭代器next方法返回Option<String>
            Some(arg) => arg,
            None => return Err("Didn't get a search string"),
        };
        let filename = match args.next() { // 迭代器next方法返回Option<String>
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        //  is_err  会返回 false 并将进行大小写不敏感搜索
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // 如果环境变量CASE_INSENSITIVE不存在，则认为是敏感的
        Ok(Config { action, filename, case_sensitive })
    }
}

// Box<dyn Error>, “通用错误类型”来包住所有错误，
// 任何实现了 std::error::Error 的类型都可以作为错误返回，这里用 trait object 做统一打包。
// 允许用 ? 统一传播不同类型的错误
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;// 如果要用 ?，函数必须返回 Result
    // println!("File contents:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.action, &contents)
    } else {
        search_case_insensitive(&config.action, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}

/// &[String] 表示一个字符串切片，它是一个指向字符串切片的引用，如果&String就是指向一个String的引用
/// &[String] 持有：起始地址 + 长度
/// pub fn parse_config(args: &[String]) -> Result<Config, &'static str> {
///     Config::new(args)
/// }
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();
    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 迭代版本
    contents.lines()
            .filter(|line| line.contains(query))
            .collect()
}


/// 大小写不敏感搜索
///
/// # Examples
///
/// ```
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.
/// ";
/// assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, &contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let query = query.to_lowercase();
    // let mut results = Vec::new();
    
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         results.push(line);
    //     }
    // }
    // results

    // 迭代版本
    let query_lowercase = query.to_lowercase();
    contents.lines()
            .filter(|line| line.to_lowercase().contains(&query_lowercase))
            .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], 
            search(query, &contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.
";

        assert_eq!(vec!["Rust:", "Trust me."], 
        search_case_insensitive(query, &contents)
        );
    }
}