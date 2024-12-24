# Sorting values with Vectors

Vectors Allow you to store more than one value in a single data structure
that puts all the values next to each other in memory.

Vectors can only store values of the same type. They are useful when you have
a list of items, such as the lines of text in a file or the prices of items in
a shopping cart.

Rust provides these two ways to reference an element so you can choose how the
program behaves when you try to use an index value outside the range of existing elements.

Let's see what happens when we try to access an element out of range:

```rust
// When we run this code, the first [] method will cause the program to panic because it references a nonexistent element. This method is best used when you want your program to crash if there’s an attempt to access an element past the end of the vector.
let does_not_exist = &v1[100];

// When the get method is passed an index that is outside the vector, it returns None without panicking. You would use this method if accessing an element beyond the range of the vector may happen occasionally under normal circumstances.
let does_not_exist = v.get(100);

thread 'main' panicked at src/vectors.rs:X:Y:
index out of bounds: the len is 3 but the index is 100
```

Recall the rule that states you can’t have mutable and immutable references in the same scope.
That rule applies in here, where we hold an immutable reference to the first element in a vector and try to add an element to the end.

This program won’t work if we also try to refer to that element later in the function.

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];  // immutable borrow occurs here

v.push(6); // mutable borrow occurs here

println!("The first element is: {first}"); // immutable borrow later used here
```

this error is due to the way vectors work: because vectors put the values next to each other
in memory, adding a new element onto the end of the vector might require allocating new memory
and copying the old elements to the new space.

In that case, the reference to the first element would be pointing to deallocated memory.
