use std::error::Error;

mod console;
mod longest;
mod shapes;
mod riddle;
mod boxpsv;
mod refcounter;
mod weakref;
mod rustthread;
mod objects;
mod post;

pub enum Tests{
    None,
    Longest,
    Riddle,
    Shapes,
    Box,
    RefCounter,
    Weakref,
    Thread,
    Object,
    Post,
}

pub fn run_tests(test: Tests) -> Result<(), Box<dyn Error>>{
    match test {
        Tests::Shapes       => shapes::rectangle_test(),
        Tests::Longest      => longest::longest_test(),
        Tests::Riddle       => riddle::riddle_test(),
        Tests::Box          => boxpsv::box_test(),
        Tests::RefCounter   => refcounter::refcounter_test(),
        Tests::Weakref      => weakref::weakref_test(),
        Tests::Thread       => rustthread::thread_test(),
        Tests::Object       => objects::object_test(),
        Tests::Post         => post::post_test(),
        _                   => {
            run_none_unsafe();
        },
    }
    
    Ok(())
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

#[cfg(test)]
mod tests{
    use crate::run_none_unsafe;

    // use super::*;
    
    #[test]
    fn it_work(){
        let gopa = run_none_unsafe();
        assert_eq!(gopa, 8);
    }
}

