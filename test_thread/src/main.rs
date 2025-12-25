use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;
use std::sync::mpsc;
use std::sync::{Mutex, Arc};
use std::rc::Rc;

fn test1() {
    println!("=============== test1 ===============");
    let mut num : i32 = 10;
    let handle : JoinHandle<()> = thread::spawn(move ||{
        for i in 1..10 {
            num += 1;
            println!("hi number {} from the spawned thread! num: {}", i, num);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn test2() {
    println!("=============== test2 ===============");
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });
    handle.join().unwrap();
    // println!("here's a vector: {:?}", v); // 这里会报错，因为v已经被移动到线程中了
}

fn test3() {
    println!("=============== test3 ===============");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
        // println!("val: {}", val); // 这里会报错，因为val已经被移动到线程中了
    });
    let received = rx.recv().unwrap();
    println!("Got: {}", received); // 这里会阻塞，直到收到消息
    handle.join().unwrap();
}

fn test4() {
    println!("=============== test4 ===============");
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("hello"),
            String::from("world"),
            String::from("rust"),
            String::from("is"),
            String::from("fun")
        ];
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_millis(1));
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx { // 迭代器会阻塞，直到收到消息
        println!("Got: {}", received);
    }
}

fn test5() {
    println!("=============== test5 ===============");
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone(); // 克隆发送者，可以发送多个消息
    let handle = thread::spawn(move || {
        let vals = vec![
            String::from("<thread1> hi"),
            String::from("<thread1> from"),
            String::from("<thread1> the"),
            String::from("<thread1> thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
            thread::sleep(Duration::from_millis(1));
        }
    });
    let handle2 = thread::spawn(move || {
        let vals = vec![
            String::from("<thread2> hello"),
            String::from("<thread2> from"),
            String::from("<thread2> the"),
            String::from("<thread2> thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            // thread::sleep(Duration::from_secs(1));
            thread::sleep(Duration::from_millis(1));
        }
    });
    for received in rx {
        println!("Got: {}", received);
    }
    handle.join().unwrap();
    handle2.join().unwrap();
}

fn test6() {
    // Mutex<T>  是一个智能指针
    println!("=============== test6 ===============");
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap(); // lock方式获取锁，lock  调用 返回 一个叫做MutexGuard  的智能指针
        *num = 6;
    }
    println!("m: {:?}", m);
}

fn test7() {
    println!("=============== test7 ===============");
    // let counter = Rc::new(Mutex::new(0)); // Rc没有使用原语，并不是线程安全的
    let counter = Arc::new(Mutex::new(0)); // Arc是原子引用计数，线程安全的
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {:?}", *counter.lock().unwrap());
}

fn main() {
    test1(); // spawn
    test2(); // move
    test3(); // channel
    test4(); // vec send
    test5(); // multi sender
    test6(); // shared state
    test7(); // mutex guard
}