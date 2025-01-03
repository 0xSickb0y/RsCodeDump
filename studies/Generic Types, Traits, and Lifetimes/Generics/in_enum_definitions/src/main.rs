#![allow(unused)]
// By using the Option<T> enum, we can express the abstract concept of an optional value.
enum Option<T> { // Option is generic over type T, and has two variants:
    Some(T), // `Some` which holds one value of type T
    None, // `None` doesn't hold any value.
}

//  This definition makes it convenient to use the Result enum anywhere we have an operation that might succeed 
// (return a value of some type T) or fail (return an error of some type E).
enum Result<T, E> { // Result is generic over two types, T and E.
    Ok(T), // Ok holds a value of type T
    Err(E), // Err holds an value of type E 
}

fn main() {}
