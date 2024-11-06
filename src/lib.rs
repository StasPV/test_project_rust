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
        Tests::Shapes => run_rectangle_test(),
        Tests::Longest => run_longest_test(),
        Tests::Riddle => run_riddle_test(),
        Tests::Box => boxpsv::box_test(),
        Tests::RefCounter => refcounter::refcounter_test(),
        Tests::Weakref => weakref::weakref_test(),
        Tests::Thread => rustthread::thread_test(),
        Tests::Object => objects::object_test(),
        Tests::Post => post::post_test(),
        _ => println!("Реализация данного теста отсутствует!"),
    }
    
    Ok(())
}

fn run_rectangle_test(){
    shapes::rectangle_test();
    console::pause_console();
}

fn run_longest_test(){
    longest::longest_test();
    console::pause_console();
}

fn run_riddle_test(){
    riddle::riddle_test();
}

#[cfg(test)]
mod tests{
    // use super::*;
    
    #[test]
    fn it_work(){
        let gopa = 2;
        assert_eq!(gopa, 2);
    }
}

