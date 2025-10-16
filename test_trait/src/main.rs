
pub trait Summary { // 要求必须实现
    // fn summarize(&self) -> String; // 只是一个签名，不是具体实现
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // 默认实现
    }

    fn summarize_author(&self) -> String;
}
pub trait Display {
    fn display(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Display for NewsArticle {
    fn display(&self) -> String {
        format!("Display {}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn display(&self) -> String {
        format!("Display {}, by {} ({})", self.username, self.content, self.reply)
    }
}

pub struct Tweet1 {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet1 {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

fn test1() {
    println!("===test1==================");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
    };
    println!("New article available! {}", article.summarize_author());

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize_author());

    let tweet1 = Tweet1 {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet1: {}", tweet1.summarize_author());
}

pub fn notify(item: &impl Summary) { // 使用impl Summary 作为参数，表示任何实现了Summary trait的类型都可以作为参数
    println!("[notify] Breaking news! {}", item.summarize());
}

pub fn notify1<T: Summary>(item: &T) { // 使用<T: Summary> 作为参数，表示只有实现了Summary trait的类型才可以作为参数
    println!("[notify1] Breaking news! {}", item.summarize());
}

// trait bound 语法
pub fn notify2<T: Summary>(item1: &T, item2: &T) { // 使用<T: Summary + Display> 作为参数，表示只有实现了Summary和Display trait的类型才可以作为参数
    println!("[notify2] Breaking news! {}", item1.summarize());
    println!("[notify2] Breaking news! {}", item2.summarize());
}

// trait bound 语法
pub fn notify3<T: Summary + Display>(item1: &T, item2: &T) { // 使用<T: Summary + Display> 作为参数，表示只有实现了Summary和Display trait的类型才可以作为参数
    println!("[notify3] Breaking news! {}", item1.summarize());
    println!("[notify3] Breaking news! {}", item2.display());
}

// trait bound 语法
pub fn notify4<T, U>(item1: &T, item2: &U)
where
    T: Summary + Display,
    U: Summary,
{
    println!("[notify4] Breaking news! {}", item1.display());
    println!("[notify4] Breaking news! {}", item2.summarize());
}

fn returns_summarizable() -> impl Summary { // 返回一个实现了Summary trait的类型，只能返回一个类型，不能返回多个类型
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
    // 不能返回多个类型，因为impl Summary 只能返回一个类型，不能返回多个类型
    // NewsArticle {
    //     headline: String::from("Penguins win the Stanley Cup Championship!"),
    //     location: String::from("Pittsburgh, PA, USA"),
    //     author: String::from("Iceburgh"),
    //     content: String::from("The Pittsburgh Penguins once again are the best"),
    // }
}
fn test2() {
    println!("===test2==================");
    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best"),
    };
    let article1 = NewsArticle {
        headline: String::from("Trump's latest article1"),
        location: String::from("USA"),
        author: String::from("Trump"),
        content: String::from("Trump's latest article1"),
    };
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    notify(&article);
    notify1(&article);
    notify2(&article, &article1);
    notify3(&article, &article1);
    notify4(&article, &tweet);
    let tweet2 = returns_summarizable();
    println!("tweet2: {}", tweet2.summarize());
}

fn main() {
    test1(); // 定义和实现trait
    test2(); // 使用trait作为参数
    // test3(); // trait bound
}
