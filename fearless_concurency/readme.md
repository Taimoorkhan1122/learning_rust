### Do not communicate by sharing memory; instead, share memory by communicating.

# Fearless Concurency

## Using Threads to Run Code Simultaneously

creating a new thread with `thread::spawn` creates a new thread. We can spawn sleep each thread to allow other threads to take turn.

We can fix the problem of the spawned thread not getting to run, or not getting to run completely, by saving the return value of `thread::spawn` in a variable. The return type of thread::spawn is `JoinHandle`. A `JoinHandle` is an owned value that, when we call the join method on it, will wait for its thread to finish.
    
- Note `handle.join().unwrap();` blocks the thread from executing or exiting until the thread represented by the handle terminates. any code after this code will be stopped until the handled thread is finished!.

We can `move` keywork inside our thread::spawn() method to enforce closure to take ownership of captured data.  

## Using a Channels to Parse message between threads
we use `mpsc::channel` function; mpsc stands for multiple producer, single consumer. In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

# Shared State concurrency
Mutex (mutual exclusion), a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data

### Rules to follow while using mutex:
- You must attempt to acquire the lock before using the data.
- When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.

### API of Mutex<T>
`Mutex<T>` is a smart pointer, call to `lock` returns a `MutexGaurd` smart pointer that wraps the value inside a `LockResult` handled with unwrap or other Result handling techniques.

## Sharing data between threads using Mutex
To share data owned by Mutex between different threads we can use `Rc<T>` whick keep tracks of refrences to multiple owners of the same value but sharing Rc<Mutex<T>> is not safe as it does not gaurantees which thread will incease the refrence count

For this purpose, we will use Arc<T> (Atomic Refrence Counting). A stands for atomic. Atomic types provide primitive shared-memory communication between threads, and are the building blocks of other concurrent types.

Atomic variables are safe to share between threads (they implement Sync) but they do not themselves provide the mechanism for sharing and follow the threading model of Rust. The most common way to share an atomic variable is to put it into an Arc (an atomically-reference-counted shared pointer).
