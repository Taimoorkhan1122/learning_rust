/*
Validating refrences with lifetimes
*/

struct SomeRefStruct<'a> {
    slice: &'a str
}

fn main() {
    let str1 = String::from("fooo");
    let str2  = "ƒoo";
    let result;

    
    result = SomeRefStruct {
        slice: longest(&str1, str2)
    };
    println!("The longest string is {:->10}\"{}\"", " ",result.slice)
    // assert_eq!("ƒoo".chars().count(),4);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }else {
        return y
    }
}