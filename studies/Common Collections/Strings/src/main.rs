#![allow(dead_code, unused)]

use std::fmt::format;
// `String` is implemented as a wrapper around a vector of bytes with extra guarantees, restrictions, and capabilities. 
fn main() {
    let mut x = String::new(); // Creates a new empty string

    let data = "String Contents"; // Creates a string literal
    println!("data: &str = {}", data);
    
    let s = data.to_string(); // .to_string() method is available on any type that implements the `Display` trait, as string literals do. 
    println!("s: String = {}", s);

    let y = "String contents".to_string(); // The method also words on a literal directly

    let z = String::from("String contents"); // We can also use the ::from() function to create a `String` from a literal.

    // Strings are UTF-8 encoded, so we can include any properly encoded data in them.
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    // We can grow a String by using the push_str method to append a string slice.
    // The .push_str() methos takes a string slice because we don't necessarily want to take ownership of the parameter.
    let mut s = String::from("foo");
    s.push_str("bar");

    // The .push() method takes a single character as a parameter.
    s.push('l');

    // Concatenation with + and format!()
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used, the reason for this is that the + operator uses the `add(self, s: &str)` method.
                      // The compiler can "coerce" the &String argument into an &str, When we call the add(self, s: &str) method, Rust uses deref coercion, which turn &s2 into &s2[..]
                     // Because add(self, s: &str) doesnt take ownership of the s parameter, s2 will still be a valid String after this operation
                    // In contrast, add(self, s: &str) does take ownership of self, as noted by the function signature.

    // For combining strigns in more complicated ways, we can use the format!() macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
    // This version is much easier to read, and the macro uses references so that this call doesn't take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}");
    println!("{}", s);

    // Rust Strings does not support indexing. A string is a wrapper over a Vec<u8>
    let hello = String::from("stri"); // Vector storing the string is 4 bytes long.
    let hello = String::from("Здравствуйте"); // Vector storing the string is 24 bytes long, the number it takes to encode "Здравствуйте" in UTF-8.
                                             // Each Unicode scalar value in that string takes 2 bytes of storage.
                                            // Therefore, Strings indexing will not always correlate to a valid Unicode scalar value.
                                           // If &"hello"[0] were valid code that returned the byte value, it would return 104, not h.

    // There are three relevant ways to look at strings from Rust’s perspective: as bytes, scalar values, and grapheme clusters
    // If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values.
    let v: [u8; 18] = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135];
    let v = ['न', 'म', 'स', '्', 'त', 'े']; // Unicode Scalar values, i.e. Rust's `char` type.
                                          // There are six char values here, but the fourth and sixth are not letters: they’re diacritics that don’t make sense on their own.
                                         // If we look at them as grapheme clusters, we’d get what a person would call the four letters that make up the Hindi word       
    let v = ["न", "म", "स्", "ते"];                                       
                                        
    // Indexing operations are expected to always take constant time (O(1)).
    // But it isn’t possible to guarantee that performance with a String, because Rust would have to walk through the contents
    // from the beginning to the index to determine how many valid characters there were.                         

    // Slicing Strings
    // Indexing into a string is often a bad idea because it's not clear what the return type of the string-indexing operation should be.
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // Rather than indexing using [] with a single number, you can use [] with a range to create a string slice containing particular bytes
                         // Here, `s` will be a &str of the first 4 bytes of the string, in this case: s = Зд
                        // If we were to try to slice only part of a character’s bytes with something like &hello[0..1], 
                       // Rust would panic at runtime in the same way as if an invalid index were accessed in a vector
    println!("First 4 bytes of Здравствуйте: {}", s);

    // Methods for iterating over Strings
    for c in "Зд".chars() { // For individual Unicode scalar values, use the chars method.
        println!("{}", c);
    }

    for c in "Зд".bytes() { // Alternatively, the .bytes() method returns each raw byte. Valid Unicode scalar values may be made up of more than one byte.
        println!("{}", c);
    }
}
