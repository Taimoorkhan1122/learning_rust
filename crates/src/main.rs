/*
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

The opt-level setting controls the number of optimizations Rust will apply
to your code, with a range of 0 to 3. Applying more optimizations extends
compiling time, so if you’re in development and compiling your code often,
you’ll want faster compiling even if the resulting code runs slower. That
is the reason the default opt-level for dev is 0. When you’re ready to
release your code, it’s best to spend more time compiling. You’ll only
compile in release mode once, but you’ll run the compiled program many
times, so release mode trades longer compile time for code that runs faster.
That is why the default opt-level for the release profile is 3.
*/

use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;

// let (tx, rx) = channel();

// let sender = thread::spawn(move || {
//     tx.send("Hello, thread".to_owned())
//         .expect("Unable to send on channel");
// });

// let receiver = thread::spawn(move || {
//     let value = rx.recv().expect("Unable to receive from channel");
//     println!("{}", value);
// });

// sender.join().expect("The sender thread has panicked");
// receiver.join().expect("The receiver thread has panicked");

fn main() {
    let (tx, rx) = channel();
    
    let sender = thread::spawn(move || {
        tx.send("Message from sender thread")
        .expect("failed to send message")
    });
    println!("sleeping for 1 sec");
    thread::sleep(Duration::from_secs(1));

    let reciever =  thread::spawn(move || {
        let value = rx.recv().expect("failed to recive message");
        println!("{}", value)
    });

    sender.join().expect("sender thread panicked");
    reciever.join().expect("reciever thread panicked");

}
