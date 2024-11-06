
pub fn box_test(){
    println!("Урок по умным указателям.");

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, *y, "Проверка равенства 5 и значения по указателю {}", *y);

    let name = MyBox::new("Stanislav");
    hello(&name);

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))));
    println!("Значение по указателю: {list:?}.");
}

fn hello(name: &str){
    println!("Hello, {name}");
}

#[derive(Debug)]
enum List{
    Cons(i32, Box<List>),
    Nil,
}

use std::ops::Deref;
struct MyBox<T>(T);

impl <T> MyBox<T>{
    fn new(x: T)-> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target{
        &self.0
    }
}