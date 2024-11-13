use std::process;
use test_project_rust::{
    self as tpr, boxpsv::BoxTest, longest::LongestTest, objects::Objects, post::PostTest, refcounter::RefCounters, riddle::Riddle, rustthread::ThreadTest, shapes::ShapesTest, weakref::WeakTest
};

fn main() {
    if let Err(e) =  tpr::run_test::<RefCounters>(){
        eprintln!("Application error: {e}");
        process::exit(1)
    }
}

