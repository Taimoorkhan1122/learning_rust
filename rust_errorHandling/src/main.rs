/*
Rust has two types of errors
1. Recoverable -> e.g file not found
2. Non Recoverable -> e.g indexing arrays out of bound
Rust handles these errors in two ways. Recoverable erros are handled
by Result<T, E> type and Non Recoverable errors are handled by panic! macro
that stops the execution of the code.

*/
fn main() {
    println!("Hello, Erorrs! Panic");
}
