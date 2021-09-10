# Iterators and Closures 

Closures are anonymous functions that can capture values from its environment. They are similar to anonymouse functions in JS

# Closure Type Inference and Annotation

Closures don’t require you to a nnotate the types of the parameters or the return value like fn functions do.
Closure definitions will have one concrete type inferred for each of their parameters and for their return value

# Storing Closures Using Generic Parameters and the Fn Traits

We will create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. This pattern is known as memoization or lazy evaluation.

# Capturing the environment with the closure

 FnOnce consumes the variables it captures from its enclosing scope, 
 known as the closure’s environment. To consume the captured variables, 
 the closure must take ownership of these variables and move them into 
 the closure when it is defined. The Once part of the name represents 
 the fact that the closure can’t take ownership of the same variables 
 more than once, so it can be called only once.
 
 FnMut can change the environment because it mutably borrows values.
 Fn borrows values from the environment immutably.

 If we want to force the closure to take the ownership of the captured 
 variable we can use `move` keyword before the parameter list.

 # Iterators

 The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up

 ## Adaptors Methods

 methods defined on the Iterator trait, known as iterator adaptors, allow you to change iterators into different kinds of iterators. Like `map` and  `filter` methods that takes and iterator and returns a returns new iterator  containing values handle by logice defined inside closure.  