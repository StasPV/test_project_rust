#[derive(Debug)]
struct Gopa{
    name: String,
    age: u32,
    len: u32,
}

fn main(){
    let gopa = Gopa{
        name: String::from("Gopa new form"),
        age: 50,
        len: 1240
    };
    let arr = get_memory(&gopa);
    let len = arr.len();

    let new_str:[u8; 4] = [71,111,112,97];
    let new_str2 = [0, 3, 208, 42, 139, 1, 0, 0].as_ptr();
    println!("{:?} - {:?} - {:?}", &(gopa.name.as_bytes()), String::from_utf8_lossy(&new_str),String::from(&new_str2));
    // unsafe{stc = std::mem::transmute::<&[u8], Gopa>(arr)};
    println!("{:?}, {:?}, {:08x}", arr, len, gopa.len);
}

fn get_memory<'a, T>(input: &'a T)->&'a [u8]{
    unsafe{
        std::slice::from_raw_parts(input as *const _ as *const u8, std::mem::size_of::<T>())
    }
}