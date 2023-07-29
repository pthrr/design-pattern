use std::cell::RefCell;

pub trait Observer<T> {
    fn notify(&mut self, value: &T);
}

pub struct Counter<'a> {
    value: usize,
    observers: Vec<&'a RefCell<dyn Observer<usize>>>,
}

impl<'a> Counter<'a> {
    pub fn new() -> Counter<'a> {
        Counter {
            value: 0,
            observers: vec![],
        }
    }

    pub fn register(&mut self, observer: &'a RefCell<dyn Observer<usize>>) {
        self.observers.push(observer);
    }

    pub fn run(&mut self) {
        loop {
            self.value += 1;

            for observer in self.observers.iter() {
                observer.borrow_mut().notify(&self.value);
            }
        }
    }
}

struct Foo;

impl Observer<usize> for Foo {
    fn notify(self: &mut Foo, value: &usize) {}
}

fn main() {
    let observer = Foo;

    let mut counter_1 = Counter::new();
    let mut counter_2 = Counter::new();
    let mut counter_3 = Counter::new();

    counter_1.register(&observer);
    counter_2.register(&observer);
    counter_3.register(&observer);

    counter_1.run();
    counter_2.run();
    counter_3.run();
}
