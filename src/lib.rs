use std::error::Error;

mod console;
mod longest;
mod shapes;
mod riddle;

pub fn run_tests() -> Result<(), Box<dyn Error>>{
    run_rectangle_test();
    run_longest_test();
    run_riddle_test();

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
