use std::process;
use test_project_rust::Tests;
use test_project_rust as tpr;

fn main() {
    if let Err(e) =  tpr::run_tests(Tests::Post){
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

