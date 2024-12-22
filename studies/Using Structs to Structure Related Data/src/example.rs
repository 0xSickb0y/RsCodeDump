// Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct.
// To do that, we add the outer attribute #[derive(Debug)] just before the struct definition

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Another way to print out a value using the Debug format is to use the dbg! macro
// this macro takes ownership of an expression (as opposed to `println!`, which takes a reference)
// Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout).
fn area2() {
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect2);
}

pub fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );


    // hen we have larger structs, it’s useful to have output that’s a bit easier to read; 
    // in those cases, we can use {:#?} instead of {:?} in the println! string.
    println!("Struct: {:#?}", rect1);

    area2();
}