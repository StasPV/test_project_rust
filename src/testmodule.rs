pub trait TestModule{
    fn new()->Self;
    fn run(&self);
 }