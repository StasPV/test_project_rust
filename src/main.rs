use std::process;

fn main() {
    if let Err(e) =  test_project_rust::run_tests(){
        eprintln!("Application error: {e}");
        process::exit(1);        
    }
}

