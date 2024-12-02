use std::mem::transmute;
use crate::BaseObject;

pub struct Simple;
impl BaseObject for Simple{
   fn new()->Self {
       Simple{}
   }

   fn run(&self) {
       let a: i64 = 42;
       let a_ptr = &a as *const i64;
       let a_addr: usize = unsafe {
           transmute(a_ptr)
       };
       println!("a: {} ({:p}...0x{:x})", a, a_ptr, a_addr+7);

       let ptr = 42 as *const Vec<String>;
       unsafe {
        let new_addr = ptr.offset(4);
        println!("{:p} -> {:p}", ptr, new_addr);
       }
   }
}

#[allow(dead_code)]
fn compare_endian() {
    let big_endian: [u8; 4] = [0xaa, 0xbb, 0xcc, 0xdd];
    let little_endian: [u8; 4] = [0xdd, 0xcc, 0xbb, 0xaa];
    let a: i32 = unsafe{ transmute(big_endian)};
    let b: i32 = unsafe{ transmute(little_endian)};

    println!("{} vs {}", a,b);
} 


#[cfg(test)]
mod simple_tests{

    fn run_none_unsafe()-> i32{
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
     
     fn run_test_abs()-> i32{
        let mut num = -run_none_unsafe();
        unsafe{
            num = abs(num);
        }
        println!("Абсолютное значение для -{num} = {num}");
        return num;
     }
         
    #[test]
    fn it_work(){        
        let gopa = run_none_unsafe();
        assert_eq!(gopa, 8);
    }

    #[test]
    fn test_abs(){
        let res = run_test_abs();
        assert_eq!(res, 3);
    }
}