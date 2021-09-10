
fn main() {
    let vec= vec![0,1,2,3];
    let mut v1_iter = vec.iter();
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());
    println!("{:?}", v1_iter.next());

    let sum : i32 = v1_iter.sum();
    println!("{}", sum);

    let mapped: Vec<_> = vec.iter().map(|x| x+1).collect();
    println!("{:?}",mapped)

}