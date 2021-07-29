/*
Vectors -> stores variable number of values next to each other
Strings -> Collection of characters
HashMaps -> stores key values pairs
*/
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
    for i in &mut vec2 {
        // we can not directly manipulate refreced data, so we have to derefrence it first 
        if *i == 45 {
            *i += 12
        }
        println!("{}", i);
    }
}
