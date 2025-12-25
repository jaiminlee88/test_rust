use test_ood::AveragedCollection;
use test_ood::{Screen, Button, SelectBox};
use test_ood::Draw;
use test_ood::Post;

fn test1() {
    println!("=======================test1=========================");
    let mut collection = AveragedCollection::new();
    collection.add(1);
    println!("average: {}", collection.average());
    collection.add(2);
    collection.add(3);
    println!("average: {}", collection.average());
    collection.remove();
    println!("average: {}", collection.average());
}

fn test2() {
    println!("=======================test2=========================");
    let screen = Screen {
        components: vec![
            Box::new(Button { 
                width: 100, 
                height: 50, 
                label: vec![
                    "Click me".to_string(),
                    "Click me2".to_string()]
                }),
            Box::new(SelectBox { 
                width: 80, 
                height: 20, 
                options: vec![
                    "Yes".to_string(), 
                    "No".to_string()]
                }),
        ],
    };
    screen.run();
}

fn test3() {
    println!("=======================test3=========================");
    let mut post = Post::new();
    
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();  
    assert_eq!("", post.content());

    post.approve();
    assert_eq!(post.content(), "I ate a salad for lunch today");
}

fn main() {
    test1(); // test OOD
    test2(); // test trait object
    test3(); // test state pattern
}
