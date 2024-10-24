use std::io;
// use std::io::prelude::*;

pub fn pause_console(){
    let stdin = io::stdin();
    println!("Нажмите любую клавишу...");
    let _ = stdin.read_line(&mut String::new()).unwrap();
}
