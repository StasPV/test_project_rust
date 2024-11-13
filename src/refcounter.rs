use crate::TestModule;
use std::{cell::RefCell, rc::Rc};
use List::{Cons, Nil};

pub struct RefCounters;
impl TestModule for RefCounters {
    fn new() -> Self {
        RefCounters {}
    }

    fn run(&self) {
        println!("Урок по счетчику ссылок!");

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
        println!("A initial rc count = {}", Rc::strong_count(&a));
        println!("A next item = {:?}", a.tail());

        let b = Rc::new(Cons(5, RefCell::new(Rc::clone(&a))));
        println!("A rc count after B creation = {}", Rc::strong_count(&a));
        println!("B initial rc count = {}", Rc::strong_count(&b));
        println!("B next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }
        println!("B rc count after changing A = {}", Rc::strong_count(&b));
        println!("A rc count after changing A = {}", Rc::strong_count(&a));

        // let value = Rc::new(RefCell::new(5));
        // let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
        // let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        // let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));
        // *value.borrow_mut() += 10;
        // println!("Текущее значение А: {a:?}");
        // println!("Текущее значение B: {b:?}");
        // println!("Текущее значение C: {c:?}");
    }
}

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentge_of_max = self.value as f64 / self.max as f64;
        match percentge_of_max {
            percent if percent >= 1.0 => self.messenger.send("Ошибка, вы превысили квоту"),
            percent if percent >= 0.95 => self
                .messenger
                .send("Срочное предупреждение, вы израсходовали более 95% квоты"),
            percent if percent >= 0.75 => self
                .messenger
                .send("Предупреждение, вы израсходовали более 75% квоты"),
            _ => (),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::refcounter::LimitTracker;
    use std::cell::RefCell;

    use crate::refcounter::Messenger;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, msg: &str) {
            self.sent_messages.borrow_mut().push(String::from(msg));
        }
    }

    #[test]
    fn it_send_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_track = LimitTracker::new(&mock_messenger, 100);
        limit_track.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}
