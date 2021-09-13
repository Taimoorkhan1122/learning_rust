use rand::Rng;

pub fn add_one(x: i32) -> i32 {
    x + rand::thread_rng().gen_range::<i32,_>(1..1000)
}
