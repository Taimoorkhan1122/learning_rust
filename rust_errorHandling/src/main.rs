/*
Rust has two types of errors
1. Recoverable -> e.g file not found
2. Unrecoverable -> e.g indexing arrays out of bound
Rust handles these errors in two ways. Recoverable erros are handled
by Result<T, E> type and Non Recoverable errors are handled by panic! macro
that stops the execution of the code.

*/

use std::fs::File;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    File::open("hello.text")?;
    Ok(())
    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(e) => match e.kind() {
    //         ErrorKind::NotFound => {
    //             println!(
    //                 "File not found! An empty file has been created file: {:?}",
    //                 e
    //             );
    //             match File::create("hello.txt") {
    //                 Ok(file) => file,
    //                 Err(e) => panic!("error creating file. File: {:?}", e),
    //             }
    //         },
    //         other => panic!("Failed to create file: {:?}",other),
    //     },
    // };
}
