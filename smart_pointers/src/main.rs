use std::fmt;
use std::ptr;
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

// impl fmt::Display for List {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f,"({})",self)
//     }
// }

use crate::List::{Cons, Nil};
use std::ops::Deref;

// ======== TREE Data Structure ===========
use std::cell::RefCell;
#[derive(Debug)]
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
}

#[derive(Debug)]
struct MyBox<T: fmt::Display>(T);

impl<T: fmt::Display> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: fmt::Display> Deref for MyBox<T> {
    type Target = T; // associated type
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: fmt::Display> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("value droped : {}", self.0)
    }
}

fn main() {
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    // println!("{}", list);
    // let x = 15;
    // let y = MyBox::new(x);
    // hello(&MyBox::new(String::from("Taimoor")));
    // drop(y);
    // println!("deref {}", *y) //  compiler calls *y -> *(y.deref())

    //     let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //     println!("Count after creating: {}", Rc::strong_count(&a));
    //     let b = Cons(4,  Rc::clone(&a));// Rc::clone increase the refrence counts
    //     println!("Count after cloning into b: {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(3, Rc::clone(&a));
    //     println!("Count after cloning into c: {}", Rc::strong_count(&a));
    // }
    // println!("Count after c goes out: {}", Rc::strong_count(&a));

    let leaf = Rc::new(Node {
        value: 3,
        children: RefCell::new(vec![Rc::new(Node {
            value: 2,
            children: RefCell::new(vec![]),
        })]),
    });

    let anotherLeaf1 = String::from("hello");
    let anotherLeaf2 = String::from("world");
    let mut ptr1 = &anotherLeaf1 as *const _;
    let mut ptr2 = &anotherLeaf2 as *const _;
    println!("ptr1 {:?} --- ptr2 {:?}", ptr1, ptr2);
    let tmp = ptr1;
    ptr1 = ptr2;
    ptr2 = tmp;

    let branch = RefCell::new(Node {
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    println!("{:#?}", branch);
    unsafe {

        println!(
            "pointer of leaf {:x?} -> {:?}\n pointer of another leaf {:?}  -> {:?}",
            &anotherLeaf1 as *const _, *ptr1, &anotherLeaf2 as *const _, *ptr2
        )
    }
}

fn hello(name: &str) {
    // this is Deref coercion that converts a type into reference to another type
    println!("Hello {}", name)
}
