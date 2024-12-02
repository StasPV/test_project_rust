use std::error::Error;

mod testmodule;
use testmodule::BaseObject;

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
pub mod simpletest;
pub mod chip8;

pub fn run_test<T: BaseObject>() -> Result<(), Box<dyn Error>>{
    T::new().run();

    Ok(())
}

#[cfg(test)]
mod tests{
    use super::*;
    
    #[test]
    fn it_work(){        
        let gopa = simpletest::run_none_unsafe();
        assert_eq!(gopa, 8);
    }

    #[test]
    fn test_abs(){
        let res = simpletest::run_test_abs();
        assert_eq!(res, 3);
    }
}

