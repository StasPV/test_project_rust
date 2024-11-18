use std::process;

use test_project_rust as tpr;

fn main() {
    if let Err(e) =  tpr::run_test::<tpr::refcounter::RefCounters>(){
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

