/*
creating module front of house 
mod (modules) let us group related defintions together
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
We can also rename external crates in cargo.TOML saying 
bar = {package="foo",  version="..."}

For making a mod public in current module but not for all we can
use pub(crate) to make it public locally.
*/
#[allow(dead_code)]
pub mod front_of_house;
use front_of_house::{hosting, serving};

#[allow(dead_code)]
pub mod back_of_house {
    
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

#[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// pub use front_of_house::hosting;
pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // using use keyword we brought the path into the scope
    hosting::add_to_waitlist();
    serving::take_order();

    let mut meal = back_of_house::Breakfast::summer("brown");
    meal.toast = String::from("Wheat");
    let order1 = back_of_house::Appetizer::Salad;
    
    println!("I want a {} toast please, and I in appetizer bring me {:?}", meal.toast, order1)
}
