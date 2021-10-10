// Mutex
use std::{
    sync::{Arc, Mutex},
    // rc::Rc,
    thread,
};

fn main() {
    /*
    // Mutex allows to one thread to access data
    let m = Mutex::new(5);
    println!("m = {:?}", m);

    {
        // This function will block the local thread until it is
        // available to acquire the mutex. Upon returning, the
        // thread is the only thread with the lock held
        let mut new = m.lock().unwrap();
        *new = 10
    }
    // we don't need to release the lock as the when
    // MutexGuard goes out of the scop the value is automatically dropped
    println!("m = {:?}", m);
    */

    // thread-safe reference-counting pointer.
    // 'Arc' stands for 'Atomically Reference Counted'.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result : {}", *counter.lock().unwrap())
}
