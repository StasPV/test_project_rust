use std::sync::{Mutex, mpsc, Arc};
use std::thread;
use std::time::Duration;
use crate::TestModule;

pub struct ThreadTest;
impl TestModule for ThreadTest{
    fn new()->Self {
        ThreadTest{}
    }

    fn run(&self) {
        println!("Урок по многопоточности");
        cannels();
        
        println!("Mutex");
        mutex();
    }
}

fn mutex(){
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10{
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }
    println!("result: {}", *counter.lock().unwrap());
}

fn cannels() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();

    thread::spawn(move || {
        let vals = vec![
            String::from("Hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for recived in rx {
        println!("Получено сообщение: {recived}");
    }
}
