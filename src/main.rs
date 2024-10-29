use std::process;

fn main() {
    println!("полная жопа");
    if let Err(e) =  test_project_rust::run_tests(){
        eprintln!("Application error: {e}");
        process::exit(1);        
    }
}

