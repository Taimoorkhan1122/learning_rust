// use std::fmt;

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"({})",self)
//     }
// }

// use  crate::List::{Cons, Nil};
use std::ops::Deref;
#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T; // associated type
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{}", list);
    let x = 5;
    let y = MyBox::new(x);
    hello(&MyBox::new(String::from("Taimoor")));
    println!("{}", *y) //  compiler calls *y -> *(y.deref())
}

fn hello(name: &str) {
    // this is Deref coercion that converts a type into reference to another type
    println!("Hello {}", name)
}
