#[derive(Debug)]
struct Gopa{
    name: String,
    age: u32,
}

fn main(){
    let gopa = Gopa{
        name: String::from("Gopa new form"),
        age: 50,
    };
    let arr = get_memory(&gopa);
    let len = arr.len();

    let new_str:[u8; 4] = [71,111,112,97];
    println!("{:?} - {:?}", &(gopa.name.as_bytes()), String::from_utf8_lossy(&new_str));
    // unsafe{stc = std::mem::transmute::<&[u8], Gopa>(arr)};
    println!("{:?}, {:?}", arr, len);
}

fn get_memory<'a, T>(input: &'a T)->&'a [u8]{
    unsafe{
        std::slice::from_raw_parts(input as *const _ as *const u8, std::mem::size_of::<T>())
    }
}