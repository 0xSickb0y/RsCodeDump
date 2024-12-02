/*
Ownership is Rust’s most unique feature and has deep implications for the rest of the language.
It enables Rust to make memory safety guarantees without needing a garbage collector.

Ownership rules:

    - Each value in Rust has an owner.
    - There can only be one owner at a time.
    - When the owner goes out of scope, the value will be dropped.
*/

/*
The mechanics of passing a value to a function are similar to those when assigning a value to a variable.
Passing a variable to a function will move or copy, just as assignment does.
*/
fn make_copy(i: i32) {
    println!("{}", i)
}

fn take_ownsership(s: String) {
    println!("{}", s);
}

// Returning values can also transfer ownership.
fn give_ownership() -> String { // This function will move its return value into the function that calls it
    let some_string = String::from("yours"); // some_string comes into scope

    return some_string; // some_string is returned and moves out to the calling function
}

// This function takes a String and returns one
fn take_and_give_back(a_string: String) -> String { // a_string comes into scope
    return a_string; // a_string is returned and moves out to the calling function
}

/*
A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. 
Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
*/
fn calculate_lenght(d: &String) -> usize { // the signature of the function uses & to indicate that the type of the parameter s is a reference.
    return d.len();
} // Here, s goes out of scope. But because it does not have ownership of what it refers to, it is not dropped.

fn data_clone() {
    let s1 = String::from("Clone");
    let s2 = s1.clone(); // Deeply copy the heap data of s1

    println!("{} | {}", s1, s2);
}

fn data_move() {
    // integers are simple values with a known, fixed size, and these two 5 values are pushed onto the stack.
    let x = 5;  // bind 5 to x
    let _y = x; // copy x to y

    // When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. 
    // We do not copy the data on the heap that the pointer refers to.
    // To ensure memory safety, after the line let s2 = s1;, Rust considers s1 as no longer valid.
    // Therefore, Rust doesn’t need to free anything when s1 goes out of scope.
    let s1 = String::from("Data");
    let s2 = s1;

    println!("{} move.", s2);
}

fn string_heap() {
    let mut s = String::from("Hello"); // This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time.
    s.push_str(", World!"); // push_str() appends a literal to a String
    println!("{}", s); // This will print `hello, world!`
}

// A scope is the range within a program for which an item is valid.
fn main() {              // s is not valid here, it’s not yet declared
    let s = "Ownership";    // s is valid from this point forward
    println!("{}", s);

    string_heap();
    data_clone();
    data_move();

    let x = String::from("Take ownership"); // s comes into scope
    let i = 5; // i comes into scope

    take_ownsership(x); // s's value moves into the function
                       // and so is no longer valid here
                       
    make_copy(i);       // x would move into the function, but i32 is Copy
    println!("{}", i); // so it's okay to still use x afterward

    let _a = give_ownership(); // give_ownsership moves its return value into a
    let b = String::from("a_string");
    let _c = take_and_give_back(b); // b is moved into take_and_give_back, which also moves its return value into c

    let d = String::from("References");
    let d_len = calculate_lenght(&d); // The &d syntax lets us create a reference that refers to the value of s1 but does not own it. 

    println!("The lenght of {} is {}", d, d_len);

} // this scope is now over, and s is no longer valid     
 // Here, c goes out of scope and is dropped. b was moved, so nothing happens.
// a goes out of scope and is dropped.

/*
When a variable goes out of scope, Rust calls a special function for us. This function is called drop.
Rust calls drop automatically at the closing curly bracket.
*/