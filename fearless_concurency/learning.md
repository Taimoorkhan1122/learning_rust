# Fearless Concurency

## Using Threads to Run Code Simultaneously

creating a new thread with `thread::spawn` creates a new thread. We can spawn sleep each thread to allow other threads to take turn.

We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of thread::spawn is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the join method on it, will wait for its thread to finish.
    
- Note `handle.join().unwrap();` blocks the thread from executing or exiting until the thread represented by the handle terminates. any code after this code will be stopped until the handled thread is finished!.

We can `move` keywork inside our thread::spawn() method to enforce closure to take ownership of captured data.  

## Using a Channels to Parse message between threads
we use `mpsc::channel` function; mpsc stands for multiple producer, single consumer. In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.