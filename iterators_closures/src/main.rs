#[derive(Debug)]
struct Shoe {
    size: u32,
    styles: String,
}

// implementing Iterator trait
#[derive(Debug, Copy, Clone)]
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}
/*
This iterator will create an iterator over our struct counter
the actual value wil not be changed but when the iterator methods
are called on this struct it will be handled by our next method*/

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let vec = vec![0, 1, 2, 3];
    let mut v1_iter = vec.iter();
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());
    // println!("{:?}", v1_iter.next());

    // let sum : i32 = v1_iter.sum();
    // println!("{}", sum);

    // let mapped: Vec<_> = vec.iter().map(|x| x+1).collect();
    // println!("{:?}",mapped);
    let shoes = vec![
        Shoe {
            size: 10,
            styles: String::from("sneakers"),
        },
        Shoe {
            size: 13,
            styles: String::from("sandals"),
        },
        Shoe {
            size: 10,
            styles: String::from("boots"),
        },
    ];

    // let in_size = shoe_in_size(shoes, 10);
    // println!("{:#?}", in_size);

    let mut counter = Counter::new();
    for count in counter.into_iter() {
        println!("{:?}", count)
    }
}

fn shoe_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
