use std::mem::transmute;
use crate::BaseObject;

pub struct SimpleTest;
impl BaseObject for SimpleTest{
   fn new()->Self {
       SimpleTest{}
   }

   fn run(&self) {
       let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
       let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];
       let a: i32 = unsafe{ transmute(big_endian)};
       let b: i32 = unsafe{ transmute(little_endian)};

       println!("{} vs {}", a,b);
   }
} 

pub fn run_none_unsafe()-> i32{
   let mut num = 5;
   let r1 = &num as *const i32;
   let r2 = &mut num as *mut i32;
   unsafe {
       println!("r1 равно: {}", *r1);
       *r2 += 3;
       println!("r2 равно: {}", *r2*2);
   }
   println!("исходное значение num:{}", num);
   return num;
}

extern "C"{
   fn abs(input: i32)-> i32;
}

pub fn run_test_abs()-> i32{
   let mut num = -run_none_unsafe();
   unsafe{
       num = abs(num);
   }
   println!("Абсолютное значение для -{num} = {num}");
   return num;
}
