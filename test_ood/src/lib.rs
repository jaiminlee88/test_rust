
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> AveragedCollection {
        AveragedCollection {
            list: vec![],
            average: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    pub fn update_average(&mut self) {
        let total : i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}

pub trait Draw {
    fn draw(&self);  // 定义一个draw方法，所有实现Draw trait的类型都必须实现draw方法
}

pub struct Screen {
    // 因为此时，trait Draw还没有实现，不知道具体大小是什么
    // dyn 把一个 trait 变成“运行期多态”的类型（trait object），使用动态分发。虚函数表多态
    // Box本身是一个指针，是知道大小的，所以可以放在Vec中
    pub components: Vec<Box<dyn Draw>>, // 相比pub components: Vec<T>只能实现一种类型, 这里用了动态分发，可以存放任意实现了Draw trait的类型
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
// ===============================================
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: Vec<String>,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button: {}x{}", self.width, self.height);
    }
}

// ===============================================
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("Drawing a select box: {}x{}", self.width, self.height);
    }
}


// =====================state pattern==========================
pub struct Post {
    // // trait object + 动态分发
    state: Option<Box<dyn State>>, // post不关心规则，委托给state来处理，state大小不确定，但box可以确定
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})), // Draft初始状态，每个state内部会有相应动作
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(&mut self) {
        // 旧状态 → 消耗掉 → 生成新状态 → 放回去
        if let Some(s) = self.state.take() { // 获取状态到s，s是旧状态，消耗掉
            self.state = Some(s.request_review()); // 新状态放回去
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve());
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
}

trait State { // 类似纯虚函数，子类必须实现
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        "" // 默认实现，子类可以重载
    }
}

// ------------------impl State for Draft------------------
struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

// ------------------impl State for PendingReview------------------
struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

// ------------------impl State for Published------------------
struct Published {}
impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&'a self, post: &'a Post) -> &'a str {
        &post.content
    }
}