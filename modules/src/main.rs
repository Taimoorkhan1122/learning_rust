use restaurant::back_of_house::*;
use restaurant::front_of_house::{hosting, serving};

fn main() {
    let my_order = Breakfast::summer("wheat");
    
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serving::take_order();
    println!("I want a sandwhich with {} bread", my_order.toast); 
    serving::serve_order();
    serving::take_bill()
}