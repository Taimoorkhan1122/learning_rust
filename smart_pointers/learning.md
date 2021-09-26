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


## RC<T> Refrence Counted Smart pointer

In rust there could be only one owner of a value at a time, the can either be borrowed or moved, if we need to enable multiple owners to point to same value so we have to use RC<T> smart pointer in order to enable multiple parts of our program to read and read the same data.
Rc<T> will allow us to ref data from multiple owners by keeping the Refrence count so that data will be pointed by multiple owners at same time.

We use the Rc<T> type when we want to allocate some data on the heap for multiple parts of our program to read and we can’t determine at compile time which part will finish using the data last. If we knew which part would finish last, we could just make that part the data’s owner, and the normal ownership rules enforced at compile time would take effect.

## Interior Mutability Pattern
