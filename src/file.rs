//! Имитация работы с файлом
#![allow(unused_variables)]
//use std::fs::File;

use std::fmt::Display;

use rand::{thread_rng, Rng};

use crate::BaseObject;

pub struct FileTest;
impl BaseObject for FileTest {
    fn new() -> Self{
        FileTest
    }
     fn run(&self){
        println!("Задача по файловым операциям");

        let mut f1 = File::new_with_data("f1.txt", &vec![114,117,115,116,33]);
        let mut buffer = vec![];
        f1 = open(f1).unwrap();
        
        let f1_length = f1.read(&mut buffer).unwrap();
        f1 = close(f1).unwrap();

        let text = String::from_utf8_lossy(&buffer);

        println!("{:?}", f1);
        println!("{}", f1);
        println!("{} is {} bytes long", &f1.name, f1_length);
        println!("{}", text);
     }
}

#[derive(Debug, PartialEq)]
enum FileState{
    Open,
    Closed,
}

impl Display for FileState{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self{
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
struct File{
    name: String,
    data: Vec<u8>,
    state: FileState,
}

#[allow(dead_code)]
impl File{

    /// Создание нового, пустого файла
    fn new(name: &str) -> Self{
        File{
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }    
    }

    fn new_with_data(name: &str, data: &Vec<u8>)->Self{
        File{
            name: String::from(name),
            data: data.clone(),
            state: FileState::Closed,
        }    
    }
    
    fn read(&self, save_to: &mut Vec<u8>)->Result<usize, String>{
        if self.state != FileState::Open{
            return Err(String::from("File must be open for reading"));
        }
        let mut tmp = self.data.clone();
        let read_len = tmp.len();
        save_to.reserve(read_len);
        save_to.append(&mut tmp);
        Ok(read_len)
    }    
}

impl Display for File{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.name, self.state)
    }
}

fn one_in(denominator: u32)->bool{
    thread_rng().gen_ratio(1, denominator)
}

fn open(mut file: File)->Result<File, String>{
    if one_in(10_000){
        let err_msg = String::from("Permission denied");
        return Err(err_msg);
    }
    file.state = FileState::Open;
    Ok(file)
}

fn close(mut file: File)->Result<File, String>{
    if one_in(100_000){
        let err_msg = String::from("Interrupted by signal");
        return Err(err_msg);
    }
    file.state = FileState::Closed;
    Ok(file)
}


