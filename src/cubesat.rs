#![allow(unused_variables)]
use crate::BaseObject;
pub struct CubeSat;
impl BaseObject for CubeSat{
    fn new()->Self {
        CubeSat
    }

    fn run(&self) {
        println!("Проект по главе 4.");
        let base: GroundStation = GroundStation{};
        let mut mailbox = Mailbox{messages: vec![]};
        let sat_ids = fetch_sat_ids();

        for sat_id in sat_ids{
            let sat = base.connect(sat_id);
            let msg = Message{to: sat_id, content: String::from("hello")};
            base.send(&mut mailbox, msg);
        }

        let sat_ids = fetch_sat_ids();
        for sat_id in sat_ids {
            let sat = base.connect(sat_id);
            let msg = sat.recv(&mut mailbox);
            println!("{:?}: {:?}", sat, msg.unwrap().content);
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
enum StatusMessage{
    Ok,
}

#[allow(dead_code)]
fn check_status(sat_id: Satellite) ->StatusMessage{
    StatusMessage::Ok
}

#[derive(Debug)]
struct Satellite{
    id: u64,
}

impl Satellite{
    fn new(id: u64)->Self{
        Satellite{
            id,
        }
    }

    fn recv(&self, mailbox: &mut Mailbox)->Option<Message>{
        mailbox.deliver(&self)
    }
}

#[derive(Debug)]
struct Mailbox{
    messages: Vec<Message>,
}

impl Mailbox{
    fn post(&mut self, msg: Message){
        self.messages.push(msg);
    }
    
    fn deliver(&mut self, recipient: & Satellite) -> Option<Message> {
        for i in 0..self.messages.len(){
            if self.messages[i].to == recipient.id{
                let msg = self.messages.remove(i);
                return Some(msg);
            }
        }
        None
    }
}

#[derive(Debug)]
struct Message{
    to: u64,
    content: String,
}

struct GroundStation;
impl GroundStation{
    fn connect(&self, sat_id: u64)->Satellite{
        Satellite::new(sat_id)
    }

    fn send(&self, mailbox: &mut Mailbox, msg: Message){
        mailbox.post(msg);
    }
}

fn fetch_sat_ids()->Vec<u64>{
    vec![1,2,3]
}