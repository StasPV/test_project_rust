use crate::BaseObject;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub struct RustThread;
impl BaseObject for RustThread {
    fn new() -> Self {
        RustThread {}
    }

    fn run(&self) {
        println!("Урок по многопоточности");
        two_chanels();
    }
}

#[allow(dead_code)]
fn mutex() {
    let counter = Arc::new(Mutex::new(0));
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
    println!("result: {}", *counter.lock().unwrap());
}

#[allow(dead_code)]
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

#[derive(Debug)]
enum MessageType {
    Ping,
    Pong,
    Pang,
}

fn two_chanels() {
    let mut handles = vec![];
    let (request_tx, request_rx) = mpsc::channel::<MessageType>();
    let (response_tx, response_rx) = mpsc::channel::<MessageType>();
    let mut count: u32 = 1;

    request_tx.send(MessageType::Ping).unwrap();
    let handle = thread::spawn(move || loop {
        let message = match response_rx.recv() {
            Ok(value) => value,
            Err(_) => return,
        };
        println!("Канал 1. Получено сообщение: {:?} {}", message, count);
        match message {
            MessageType::Pong => match request_tx.send(MessageType::Ping){
                Err(_)=> return,
                _=>(),
            }
            // MessageType::Pang => break,
            _ => (),
        }

        count += 1;
        if count > 10 {
            request_tx.send(MessageType::Pang).unwrap();
            return;
        }
    });
    handles.push(handle);

    let handle = thread::spawn(move || loop {
        let message = match request_rx.recv() {
            Ok(value) => value,
            Err(_) => return,
        };
        println!("Канал 2. Получено сообщение: {:?}", message);
        match message {
            MessageType::Ping => {
                match response_tx.send(MessageType::Pong){
                    Err(_)=> return,
                    _=>(),
                }
            },
            MessageType::Pang => return,
            _ => (),
        }
    });
    handles.push(handle);
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Игра завершена!")
}
