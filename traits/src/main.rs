/*
Generics used to avoid code duplication by providing abstract syntax 
for concrete types. There is no runtime cost for using generics as
generics are implemented in such a way that it handles generic values using
Monomorphization, it is the process of turning generic code into
 specific code by filling in the concrete types that are used when compiled.
*/

fn main() {
    let my_arr = [10,200,102,30,15,100,15,15,25,789,9,145,32];
    println!("the greatest number is {:|^5} ", largest(&my_arr))
}

// finding greatest function with concrete type

// fn largest_num(num_arr: &[i32]) -> i32 {
//     let mut greatest = num_arr[0];
//     for &num in num_arr {
//         if num > greatest { 
//             greatest = num; 
//         }
//     }
//     return greatest;
// }

// finding greatest values function with generic types
// use std::cmp::PartialOrd; <- this is included in prelude
 
struct Point<P> {
    x:  P,
    y: P,
}

impl<P: std::ops::Mul + Copy> Point<P> {
    fn calc(&self)  -> <P as std::ops::Mul>::Output {
        let a  = self.x * self.y;
        return a;
    }
}
fn largest<T:  PartialOrd + Copy>(num_arr: &[T]) -> T {

    let p1 = Point {
        x: 1,
        y: 2,
    };
    println!("{:?} area = {:->5}", (p1.x, p1.y), p1.calc());
    
    let mut greatest = num_arr[0];
    for &num in num_arr {
        if num > greatest { 
            greatest = num; 
        }
    }
    return greatest;
}