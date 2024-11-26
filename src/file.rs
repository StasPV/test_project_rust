#![allow(unused_variables)]
//use std::fs::File;

use crate::BaseObject;

pub struct FileTest;
impl BaseObject for FileTest {
    fn new() -> Self{
        FileTest
    }
     fn run(&self){
        println!("Задача по файловым операциям");

        let mut f1 = File{
            name: String::from("f1.txt"),
            data: vec![114,117,115,116,33],
        };
        let mut buffer = vec![];
        open(&mut f1);
        
        let f1_length = read(&f1, &mut buffer);
        close(&mut f1);

        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f1);
        println!("{} is {} bytes long", &f1.name, f1_length);
        println!("{}", text);
     }
}

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
}

fn open(f: &mut File)->bool{
    true
}

fn close(f: &mut File)->bool{
    true
}

fn read(f: &File, save_to: &mut Vec<u8>)->usize{
    let mut tmp = f.data.clone();
    let read_len = tmp.len();
    save_to.reserve(read_len);
    save_to.append(&mut tmp);
    read_len
}

