use std::error::Error;

// mod testmodule;
// use testmodule::BaseObject;

pub mod longest;
pub mod shapes;
pub mod riddle;
pub mod boxpsv;
pub mod refcounter;
pub mod weakref;
pub mod rustthread;
pub mod objects;
pub mod post;
pub mod file;
pub mod cubesat;
pub mod simple;
pub mod chip8;
pub mod fview;

pub fn run_test<T: BaseObject>() -> Result<(), Box<dyn Error>>{
    T::new().run();

    Ok(())
}

pub trait BaseObject<RHS=Self>{
    fn new()->Self;
    fn run(&self){
        println!
        ("Пустая реализация типа {}", std::any::type_name_of_val(self));
    }
 }


