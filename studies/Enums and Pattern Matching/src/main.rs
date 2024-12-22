use std::io;

/*
Where structs give you a way of grouping together related fields and data, enums give you a way of saying a value is one of a possible set of values. 

Both version four and version six addresses are still fundamentally IP addresses,
so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddr enumeration and listing the possible kinds an IP address can be, V4 and v6. 
*/
enum IpAddr {
    V4(u8, u8, u8, u8), // the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    V6(String),
}

enum Message {
    _Quit, // No data associated with it at all
    _Move { x: i32, y: i32 }, // named fields, like a struct.
    Write(String), // a single string
    _ChangeColor(i32, i32, i32), // three `i32` values
}

// we’re also able to define methods on enums.
impl Message {
    fn call(&self) {
        println!("m.call()")
    }
}

/*
Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>
it is defined by the standard library as follows:

enum Option<T> {  // <T> means that the Some variant of the Option enum can hold one piece of data of any type
    None,
    Some(T),
}

*/

/*
Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
Patterns can be made up of literal values, variable names, wildcards, and many other things
*/
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

//Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants.
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin { // The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.
        Coin::Penny =>  {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}.", state);
            25
        }
    }
}

/*
we can also handle Option<T> using match, as we did with the Coin enum!
Instead of comparing coins, we’ll compare the variants of Option<T>, but the way the match expression works remains the same.
*/
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn catch_all(dice_roll: i32) {
    match dice_roll {
        3 => println!("Add fancy hat"),
        7 => println!("Remove fancy hat"),
        other => println!("Move player to position {}", other), // last pattern will match all values not specifically listed.
        // _ => println!("Reroll the dice") | `_` is a special pattern that matches any value and does not bind to that value.
        // _ => () | Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.
    }
}

fn concise_control_flow() {
    let config_max = Some(3u8);
    
    /*
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    If the value is Some, we print out the value in the Some variant by binding the value to the variable max in the pattern.
    We don’t want to do anything with the None value.
    To satisfy the match expression, we have to add _ => () after processing just one variant, which is annoying boilerplate code to add.
    
    Instead, we could write this in a shorter way using if let.
    */

    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}

fn main() {
    // Note that the variants of the enum are namespaced under its identifier.
    let _home = IpAddr::V4(127, 0, 0, 1); // IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    let _loopback = IpAddr::V6(String::from("::1"));

    let m = Message::Write(String::from("Hello, World!"));
    m.call();

    // Option<T> example
    let some_number = Some(5); // type: Option<i32> | Rust can infer these types because we’ve specified a value inside the Some variant. 
    let _some_char = Some('e'); // type: Option<char>
    let _absent_num: Option<i32> = None; // Rust requires us to annotate the overall Option type: the compiler can’t infer the type that the corresponding Some variant will hold by looking only at a None value.

    // You have to convert an Option<T> to a T before you can perform T operations with it.
    let five = some_number.unwrap(); // five: i32
    println!("5 * 10: {}", five * 10);

    // In order to have a value that can possibly be null, you must explicitly opt in by making the type of that value Option<T>. 
    let a: Option<i32> = Some(10);
    let b: Option<i32> = None;
    
    // You can handle these options explicitly
    match a {
        Some(value) => println!("a has a value: {}", value),
        None => println!("a is None"),
    }
    
    match b {
        Some(value) => println!("b has a value: {}", value),
        None => println!("b is None"),
    }

    let dime = Coin::Dime;
    let penny = Coin::Penny;
    let quarter = Coin::Quarter(UsState::Alaska);

    println!("The value of a `Dime` is: {}\n", value_in_cents(dime));
    println!("The value of a `Penny` is: {}\n", value_in_cents(penny));
    println!("The value of a `Quarter` is: {}\n", value_in_cents(quarter));

    let five = Some(5);
    println!("5 + 1 = {:?}", plus_one(five));
    println!("None + 1 = {:?}", plus_one(None));

    catch_all(9);
    concise_control_flow();
}

