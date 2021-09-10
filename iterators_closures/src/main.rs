use std::thread;
use std::time::Duration;
use std::collections::HashMap;

// A struct that is Generic over some type T which implements Trait bounds Fn

/*
 FnOnce consumes the variables it captures from its enclosing scope, 
 known as the closure’s environment. To consume the captured variables, 
 the closure must take ownership of these variables and move them into 
 the closure when it is defined. The Once part of the name represents 
 the fact that the closure can’t take ownership of the same variables 
 more than once, so it can be called only once.
 
 FnMut can change the environment because it mutably borrows values.
 Fn borrows values from the environment immutably.

 If we want to force the closure to take the ownership of the captured 
 variable we can use `move` keyword before the parameter list.
*/

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<Option<u32>, u32>,
}

// Implement this for the Cacher struct that is implements the type T: Fn(u32) -> u32

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }
    
    fn value(&mut self, args: u32) -> u32 {
        match self.value.get(&Some(args))  {
            Some(v) => *v,
            None => {
                let v = self.value.entry(Some(args)).or_insert((self.calculation)(args));
                *v
            }
        }        
    }
}

fn main() {
    let simulated_user_specified_value = 24;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

// fn simulated_expensive_calculation(intensity: u32) -> u32 {
//     println!("calculating slowly...");
//     thread::sleep(Duration::from_secs(2));
//     intensity
// }

fn generate_workout(intensity: u32, random_number: u32) {
    let x  = String::from("hello");
    
    let mut expensive_result = Cacher::new(|num| -> u32 {
        println!("calculating slowly... \n{}", hello);
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(20));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}