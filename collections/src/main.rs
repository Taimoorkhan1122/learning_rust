/*
Vectors -> stores variable number of values next to each other
Strings -> Collection of characters
HashMaps -> stores key values pairs

Rust has only one string type in core language that is str (string slice type)
String type is owned type and move occurs if referenced withoud & operator.
While the slice type implements copy traits so when refered by another variable 
copy of values occurs rather than move.
We cannot index String as their return type is unexpected, depending on the containng data
it may return a unicode scalar values, bytes or a grapheme cluster (individual letters)

Hashmaps are like distionaries or objects in other objects
Hashmaps are homogenous, if we want to store multiple values we can use enums or BOX 
https://www.simonewebdesign.it/rust-hashmap-insert-values-multiple-types/ 
*/

#[derive(Debug)]
enum Maps {
    Str(String),
    Int(i32),
    Bool(bool)
}

use std::collections::HashMap;
fn main() {
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    // using vec! macro
    let mut vec2 = vec![12,2,45,16];
    // let second = &vec2[1];
    // println!("{}", second);

    // match vec2.get(1) {
    //     Some(s) => println!("{}", s),
    //     None => println!("No item found! probably index out of bound")
    // }
    // for i in &mut vec2 {
    //     // we can not directly manipulate refreced data, so we have to derefrence it first 
    //     if *i == 45 {
    //         *i += 12
    //     }
    //     println!("{}", i);
    // }
    // let one = "1";
    // let two = one;
    // println!("one -> {:?} two -> {:?}", one.as_ptr(), two.as_ptr());
    let mut map: HashMap<String, Maps> = HashMap::new();
    map.insert("Name".to_string(), Maps::Str(String::from("Taimoor Khan")));
    map.insert("Age".to_string(), Maps::Int(22));
    map.insert("isloggedIn".to_string(), Maps::Bool(true));
 
    println!("{:?}", map.get("Age")); // returns a Result Enum
    for (key, value) in map {
        println!("the minimum {} is {:?}", key, value);
    }
}
