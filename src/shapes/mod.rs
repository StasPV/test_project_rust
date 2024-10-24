use std::io;

pub fn rectangle_test(){
    let rect1 = Rectangle{
        width: 30,
        height: 50,
    };
    let mut size = String::new();
    
    println!("Пожалуйста введите желаемый размер - ");
    io::stdin()
       .read_line(&mut size)
       .expect("Ошибка чтения строки");
    let size:u32 = size.trim().parse().expect("Пожалуйста введите число!");
    let rect2 = Rectangle::square(size);

    println!("rect1 is {rect1:?} and his area {}", rect1.area());
    println!("Can rect1 hold rect2? - {}", rect1.can_hold(&rect2));
    let config_max = Some(3u8);
    if let Some(max) = config_max{
        println!("Максимальное значение конфигурации {max}");
    }

}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32{
        self.width*self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size :u32) -> Self {
        Self { width: size, height: size }
    }
}
