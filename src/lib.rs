use std::error::Error;

mod testmodule;

pub mod longest;
pub mod shapes;
pub mod riddle;
pub mod boxpsv;
pub mod refcounter;
pub mod weakref;
pub mod rustthread;
pub mod objects;
pub mod post;


use testmodule::BaseObject;

pub fn run_test<T: BaseObject>() -> Result<(), Box<dyn Error>>{
    T::new().run();

    Ok(())
}


 pub struct SimpleTest;
 impl BaseObject for SimpleTest{
    fn new()->Self {
        SimpleTest{}
    }

    fn run(&self) {
        run_test_abs();
    }
 } 

fn run_none_unsafe()-> i32{
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    unsafe {
        println!("r1 равно: {}", *r1);
        *r2 += 3;
        println!("r2 равно: {}", *r2*2);
    }
    println!("исходное значение num:{}", num);
    return num;
}

extern "C"{
    fn abs(input: i32)-> i32;
}

fn run_test_abs()-> i32{
    let mut num = -run_none_unsafe();
    unsafe{
        num = abs(num);
    }
    println!("Абсолютное значение для -{num} = {num}");
    return num;
}
#[cfg(test)]
mod tests{
    use crate::{run_none_unsafe, run_test_abs};

    // use super::*;
    
    #[test]
    fn it_work(){
        let gopa = run_none_unsafe();
        assert_eq!(gopa, 8);
    }

    #[test]
    fn test_abs(){
        let res = run_test_abs();
        assert_eq!(res, 3);
    }
}

