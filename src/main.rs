use std::process;

fn main() {
    if let Err(e) =  test_project_rust::run_tests(){
        eprintln!("Общая ошибка приложения: {e}");
        process::exit(1);        
    }
}

