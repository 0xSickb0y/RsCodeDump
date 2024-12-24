#![allow(dead_code, unused)]

use std::vec;

fn main() {
    let v0: Vec<i32> = Vec::new(); // Create empty vector of i32, Vec<T> provided by the standard library can hold any type.

    let v1 = vec![0, 1, 2]; // `vec!` macro, creates a new vector that holds the values you give it.

    // To create a vector and then add elements to it, we can use the push method,
    let mut v2 = Vec::new();

    v2.push(0);
    v2.push(1);
    v2.push(2);

    // There a two ways to reference a value stored in a vector: with indexing syntax or the `get` method.
    let second: &i32 = &v2[1]; // usign & and [] gives us a reference to the element at the index value.
    println!("The second element from v2: {}", second);

    let third: Option<&i32> = v2.get(2); // Returns a reference to an element or subslice depending on the type of index. If given a position, returns a reference to the element at that position or None if out of bounds.
    match third {
        Some(third) => println!("The third element from v2: {}", third),
        None => println!("There is no third element"),
    }

    // Iterating over the values in a vector
    for i in &v1 {
        println!("{}", i);
    }

    // We can also iterate over mutable references in order to make changes to all the elements.
    for x in &mut v2 {
        *x += 50; // we have to use the * dereference operator to get to the value in i before we can use the += operator. 
    }

    // Using an Enum to Store multiple types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // The variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum.
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("red")),
        SpreadsheetCell::Float(10.20),
    ];

    // Using an enum plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.
    // If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object.
    for cell in &row {
        match cell {
            SpreadsheetCell::Int(value) => println!("Integer: {}", value),
            SpreadsheetCell::Float(value) => println!("Float: {}", value),
            SpreadsheetCell::Text(value) => println!("Text: {}", value),
        }
    }

    let v3 = vec![1, 2, 3, 4];
} // <- v3 goes out of scope and is freed here 

/*
Like any other struct, a vector is freed when it goes out of scope.

When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds
will be cleaned up. The borrow checker ensures that any references to contents of a vector are
only used while the vector itself is valid.
*/