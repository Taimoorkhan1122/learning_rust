use std::process;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::time::Duration;

fn main() {
    // let v = vec![1,2,3,4,5];
    // let handle = thread::spawn(move ||  {
    //     for i in v.iter() {
    //         println!("vector {:?} at {} ", v, i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });
    // for i in 1..5 {
    //     println!("Hi {} from the main thread", i);
    //     thread::sleep(Duration::from_millis(1));
    // }
    // handle.join().unwrap();

    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for v in vals {
            tx.send(v).unwrap_or_else(|e| {
                println!("error in sending message: {}", e);
                process::exit(1);
            });
            thread::sleep(Duration::from_millis(1000))
        }
    });

    // let received = rx.recv().unwrap_or_else(|e| {
    //     println!("error in parsing message: {}", e);
    //     process::exit(1);
    // });
    // println!("Got message: {}", received);

    // receiving multiple values
        let time_recv = time::Instant::now();
    for rec in rx {
        println!("Got message: {} at {:?} secs", rec, time_recv.elapsed().as_secs());
    }
}
