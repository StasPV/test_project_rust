
pub trait BaseObject<RHS=Self>{
    fn new()->Self;
    fn run(&self){
        dbg!("Пустая реализация типа {:?}");
    }
 }