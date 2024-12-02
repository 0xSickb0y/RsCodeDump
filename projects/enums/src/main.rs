use std::io;

/*
Where structs give you a way of grouping together related fields and data enums give you a way of saying a value is one of a possible set of values. 
*/


/*
Both version four and version six addresses are still fundamentally IP addresses,
so they should be treated as the same type when the code is handling situations that apply to any kind of IP address.

We can express this concept in code by defining an IpAddrKind enumeration and listing the possible kinds an IP address can be, V4 and v6. 
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

// Rust does not have nulls, but it does have an enum that can encode the concept of a value being present or absent. This enum is Option<T>
/* it is defined by the standard library as follows:

enum Option<T> {  // <T> means that the Some variant of the Option enum can hold one piece of data of any type
    None,
    Some(T),
}

*/

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

}

