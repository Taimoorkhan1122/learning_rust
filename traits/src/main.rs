/*
Validating refrences with lifetimes
The main concept is statically defining lifetime for any refrenced variable.
If we have to parameter in a function which are refrence types so the
both will get a lifetime of shortest one.

Lifetime elision -> these are set of rules that compiler follows
to apply life time anotation itself these are three rules, if compiler
is not able to infer lifetime other than these defined rules it will
throw error.
*/

struct SomeRefStruct<'a> {
    slice: &'a str
}

impl<'a> SomeRefStruct<'a> {
    fn get_chars(&self)-> Vec<char> {
        let mut mychar :Vec<char> = vec!();
        for character in self.slice.chars() {
            mychar.push(character)
        }
        return mychar;
    }
}

fn main() {
    let str1 = String::from(" ﭨ");
    let str2  = "p";
    let result;

    
    result = SomeRefStruct {
        slice: longest(&str1, str2)
    };
    
    println!("The longest string is {:->10}\"{:?}\"", " ",result.slice.chars().count());
    println!("The longest string is {:->10}\"{:?}\"", " ",result.get_chars())
    // assert_eq!("ƒoo".chars().count(),4);
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }else {
        return y
    }
}