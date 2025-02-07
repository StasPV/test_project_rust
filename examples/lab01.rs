use std::fs::File;

fn main(){
    let file = File::open("poem.txt").unwrap();
    let mut mmap_options = memmap::MmapOptions::new();
    let mmap = unsafe{mmap_options.offset(10).len(20).map(&file).unwrap()};

    println!("{:?}", String::from_utf8_lossy(&mmap[..]));
}