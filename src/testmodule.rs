
pub trait BaseObject<RHS=Self>{
    fn new()->Self;
    fn run(&self){
        println!
        ("Пустая реализация типа {}", std::any::type_name_of_val(self));
    }
 }