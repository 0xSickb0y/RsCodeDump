
/*
Structs are similar to tuples, in that both hold multiple related values.
Like tuples, the pieces of a struct can be different types.
Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
Adding these names means that structs are more flexible than tuples:
you don’t have to rely on the order of the data to specify or access the values of an instance.
*/

mod example;
mod method;

// Rust also supports structs that look similar to tuples, called tuple structs.
// Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

 /*
Here we used the owned String type rather than the &str string slice type.
This is a deliberate choice because we want each instance of this struct to own
all of its data and for that data to be valid for as long as the entire struct is valid.
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}


// You can also define structs that don’t have any fields!
// These are called unit-like structs because they behave similarly to ()
struct AlwaysEqual;

fn main() {
    // To use a struct after we’ve defined it, we create an instance of that struct by specifying concrete values for each of the fields.
    let mut user0 = User {
        active: true,
        username: String::from("sickb0y"),
        email: String::from("name@domain.xyz"),
        sign_in_count: 10
    };
    // To get a specific value from a struct, we use dot notation.
    user0.email = String::from("aname@domain.xyz");

    println!("Username: {},\nActive: {},\nEmail: {},\nsign_in_count: {}",
        user0.username,
        user0.active,
        user0.email,
        user0.sign_in_count
    );
            
    println!("\n");
            
    /* Creating Instances from Other Instances with Struct Update Syntax
    It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some.
    
    Using struct update syntax, we can achieve the same effect with less code. 
    The syntax `..` specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    */

    let user1 = User {
        email: String::from("fake_user@malicious.ru"),
        sign_in_count: 1337,
        ..user0
    };

    println!("Username: {},\nActive: {},\nEmail: {},\nsign_in_count: {}",
        user1.username,
        user1.active,
        user1.email,
        user1.sign_in_count
    );
    
    println!("\n");

    // Note that the black and origin values are different types because they’re instances of different tuple structs. 
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black: ({}, {}, {}), Origin: ({}, {}, {})",
        black.0, black.1, black.2,
        origin.0, origin.1, origin.2
    );

    /*
    Imagine that later we’ll implement behavior for this type such that every instance of 
    AlwaysEqual is always equal to every instance of any other type, perhaps to have a known result for testing purposes.
    We wouldn’t need any data to implement that behavior! 
    */

    let _subject = AlwaysEqual;

    /*
    It’s also possible for structs to store references to data owned by something else,
    but to do so requires the use of lifetimes.

    Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
    */

    println!("\n");

    // To understand when we might want to use structs, let’s write a program that calculates the area of a rectangle.
    example::main();
    method::main();
    
}
