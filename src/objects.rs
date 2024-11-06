
pub(crate) fn object_test() {
    println!("Урок по объектной технологии");
    let screen = Screen{
        components: vec![
            Box::new(
                SelectBox{
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ]
                }
            ),
            Box::new(
                Button{
                    width: 50,
                    height: 10,
                    label: String::from("Ok"),
                }
            ),
        ]
    };
    screen.run();
}

trait Draw {
    fn draw(&self);
}

struct Screen{
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen{
    fn run(&self){
        for component in self.components.iter(){
            component.draw();
        }
    }
}

struct Button{
    width: u32,
    height: u32,
    label: String,
}

impl Draw for Button{
    fn draw(&self){
        println!("Рисуем кнопку: {}:{}:{}", &self.width, &self.height, &self.label);
    }
}
struct SelectBox{
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox{
    fn draw(&self){
        println!("Рисуем область выделения: {}:{}:{:?}", &self.width, &self.height, &self.options);
    }
}