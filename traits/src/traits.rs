/*
TRAITS similar to interfaces in other languages but with some differences
Traits are used to define behaviour of a type

TRAIT bounds -> the impl trait is syntax sugar for longer form of
trait implementations called trait bounds


*/
use std::{any::type_name, fmt};
pub trait Summarize {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summarize for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summarize for Tweet {
    fn summarize(&self) -> String {
        format!(
            "Tweet by @{} \ncontent: {} \nis reply:{} \nis Retweet: {}",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}, {}, {}, {})",
            self.username, self.content, self.reply, self.retweet
        )
    }
}

// // Trait as parameter
// fn notify(item: &(impl Summarize + fmt::Display)) {
//     println!("Breaking News! {}",item.summarize());
// }

// // Trait bound syntax
// fn notify<T: Summarize + fmt::Display>(item: &T) {
//     println!("Breaking News! {}",item.summarize());
// }

// Using where clause
fn notify<T, U>(t: &T, _u: &U)
where
    T: Summarize + fmt::Display,
    U: Clone + fmt::Debug,
{
    println!("Breaking News! {}", t.summarize());
}

fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    let vev1 = 12;
    notify(&tweet, &vev1);
    let p1 = Point::new(12, 14);
    let p2 = Point::new("12", "14");
    println!("Product of point is {:?}", p1.calc());
    type_of(&p1);
    type_of(&p2);

}
/*
We are using Trait bounds to conditionally implement methods
Those types which does not implements the second trait will not be able to
access the method defined in that impl block
*/

#[derive(Debug)]
struct Point<P> {
    x: P,
    y: P,
}

impl<P> Point<P> {
    // Self (Type of current object)
    fn new(x: P, y: P) -> Self {
        Self { x, y }
    }
}

fn type_of<P>(_: &P) {
    println!("{}", type_name::<P>());
}

impl<P: std::ops::Mul<Output = P> + Copy> Point<P> {
    fn calc(&self) -> P {
        let a = self.x * self.y;
        return a;
    }
}

/*
  We can also use implement a trait for any type that satifies the trait bound
*/

// impl<T: fmt::Display> Summarize for T {

// }