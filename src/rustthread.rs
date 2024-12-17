use crate::BaseObject;
use std::fmt::Display;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

static MAX_COUNT: u32 = 57;

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

#[derive(Debug, PartialEq)]
enum Message {
    Ping(u32),
    Pong(u32),
    Pang,
}
impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Ping(counter) => write!(f, "Ping: {}", counter),
            Message::Pong(counter) => write!(f, "Pong: {}", counter),
            Message::Pang => write!(f, "PANG!"),
        }
    }
}
impl Message {
    fn get_message(id: u32, value: u32) -> Self {
        match id {
            1 => Message::Ping(value),
            2 => Message::Pong(value),
            _ => Message::Pang,
        }
    }
}

struct Actor {
    sender: Sender<Message>,
    receiver: Receiver<Message>,
    number: u32,
}
impl Actor {
    fn new(sender: Sender<Message>, receiver: Receiver<Message>, number: u32) -> Self {
        Actor { sender, receiver, number }
    }
}

fn two_chanels() {
    let mut handles = vec![];
    let (request_tx, request_rx) = mpsc::channel::<Message>();
    let (response_tx, response_rx) = mpsc::channel::<Message>();
    let actor1 = Actor::new(request_tx, response_rx, 1);
    let actor2 = Actor::new(response_tx, request_rx, 2);

    actor1.sender.send(Message::Ping(1)).unwrap();
    let handle = thread::spawn(move || {
        run_actor(&actor1);
    });
    handles.push(handle);

    let handle = thread::spawn(move || {
        run_actor(&actor2);
    });
    handles.push(handle);
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Игра завершена!")
}

fn run_actor(actor: &Actor) {
    loop {
        let message = match actor.receiver.recv() {
            Ok(value) => value,
            Err(_) => return,
        };
        println!("Канал {}. Получено сообщение: {}", actor.number, message);
        match message {
            Message::Ping(val) | Message::Pong(val) => {
                let count = val + 1;
                if count > MAX_COUNT {
                    _ = actor.sender.send(Message::Pang);
                    return;
                } else {
                    match actor
                        .sender
                        .send(Message::get_message(actor.number, count))
                    {
                        Err(_) => return,
                        _ => (),
                    }
                }
            }
            Message::Pang => return,
        }
    }
}

#[cfg(test)]
mod thread_tests{
    use crate::rustthread::Message;

    #[test]
    fn check_get_message(){
        let message = Message::get_message(1, 10);
        assert_eq!(Message::Ping(10), message);
    }
}