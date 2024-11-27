#![allow(unused_variables)]
use crate::BaseObject;
pub struct CubeSat;
impl BaseObject for CubeSat{
    fn new()->Self {
        CubeSat
    }

    fn run(&self) {
        println!("Проект по главе 4.");
        let sat_a = Satellite { id: 0 };
        let sat_b = Satellite { id: 1 };
        let sat_c = Satellite { id: 2 };
        let status_a = check_status(sat_a);
        let status_b = check_status(sat_b);
        let status_c = check_status(sat_c);

        println!("sat a:{:?}, sat b:{:?}, sat c:{:?}", status_a, status_b, status_c);

        let status_a = check_status(sat_a);
        let status_b = check_status(sat_b);
        let status_c = check_status(sat_c);

        println!("sat a:{:?}, sat b:{:?}, sat c:{:?}", status_a, status_b, status_c);
    }
}

#[derive(Debug)]
enum StatusMessage{
    Ok,
}

fn check_status(sat_id: Satellite) ->StatusMessage{
    StatusMessage::Ok
}

#[derive(Debug)]
struct Satellite{
    id: u64,
    mailbox: Mailbox,
}

#[derive(Debug)]
struct Mailbox{
    messages: Vec<Message>,
}

type Message = String;