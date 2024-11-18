use std::{
    io,
    cmp::Ordering
};
use rand::Rng;
use crate::BaseObject;

pub struct Riddle;
impl BaseObject for Riddle {
    fn new()->Self {
        Riddle{}
    }

    fn run(&self) {
        println!("Угадай число!");
        let secret_number = rand::thread_rng().gen_range(1..=100);
        let mut counter = 0;
        loop {
            let mut guess = String::new();
            
            println!("Пожалуйста введите желаемое число - ");
            
            io::stdin()
            .read_line(&mut guess)
            .expect("Ошибка чтения строки");
        //println!("Вы ввели {}", guess);
            counter += 1;
            let guess:u32 = guess.trim().parse().expect("Пожалуйста введите число!");
        
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Слишком мало!"),
                Ordering::Greater => {
                    println!("Слишком много!")
                }
                Ordering::Equal => {
                    println!("Вы выиграли! Количество попыток: {counter}");
                    break;
                }
            }
        }
    }
}
