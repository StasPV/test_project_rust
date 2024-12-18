use std::process;
// use self::tpr::*;

use test_project_rust as tpr;

fn main() {
    if let Err(e) =  tpr::run_test::<tpr::simple::Simple>(){
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

