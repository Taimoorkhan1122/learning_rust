# Smart Pointers

A pointer is a general concept for a variable that contains an address in memory. This address refers to, or “points at,” some other data.

## Box<T>

Box allows to store data on the heap while the pointer to the data remains on the stack.

Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack. But they don’t have many extra capabilities either. You’ll use them most often in these situations:

- When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
- When you have a large amount of data and you want to transfer ownership but ensure the data won’t becopied when you do so
- When you want to own a value and you care only that it’s a type that implements a particular traitrather than being of a specific type

## Implicit Deref Coercions with Functions and Methods

Deref coercion is a convenience that Rust performs on arguments to functions and methods. Deref coercion works only on types that implement the Deref trait. Deref coercion converts such a type into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str