# Common Collections

Rust’s standard library includes a number of very useful data structures called collections. Most other data types represent one specific value, but collections can contain multiple values. Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs. Each kind of collection has different capabilities and costs, and choosing an appropriate one for your current situation is a skill you’ll develop over time.

We’ll discuss three collections that are used very often in Rust programs:

- A vector allows you to store a variable number of values next to each other.
    
    
- A string is a collection of characters.

- A hash map allows you to associate a value with a specific key. It’s a particular implementation of the more general data structure called a map.
