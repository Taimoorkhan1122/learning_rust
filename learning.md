# Iterators and Closures 

Closures are anonymous functions that can capture values from its environment.

# Closure Type Inference and Annotation

Closures don’t require you to a nnotate the types of the parameters or the return value like fn functions do.
Closure definitions will have one concrete type inferred for each of their parameters and for their return value

# Storing Closures Using Generic Parameters and the Fn Traits

We will create a struct that will hold the closure and the resulting value of calling the closure. The struct will execute the closure only if we need the resulting value, and it will cache the resulting value so the rest of our code doesn’t have to be responsible for saving and reusing the result. This pattern is known as memoization or lazy evaluation.  

# Capturing the environment with the closure

