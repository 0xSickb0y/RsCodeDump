#![allow(unused)]
// Lifetime Annotation Syntax
//
// &i32        -> A reference
// &'a i32     -> A reference with an explicit lifetime
// &'a mut i32 -> A mutable reference with an explicit lifetime
//


fn lifetime() { // visual representation of the variables lifetimes
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+

// The returned reference will be valid as long as both the parameters are valid. 
// The lifetime of the reference returned is the same as the smaller of the lifetimes of the values referred to by the function arguments.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// always return the first parameter, no need to specify a lifetime for `y`
fn return_first<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 { // we’re not required to annotate the lifetime of the reference to self because of the first elision rule.
        3
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { // Rust applies the first lifetime elision rule and gives both &self and announcement their own lifetimes.
        println!("Attention please: {announcement}"); // 
        self.part //  because one of the parameters is &self, the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    }
}

// Lifetime Elision
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// all in one implementation of Generics, Trait bounds, and lifetimes.
// Import the Display trait, which is necessary for the 'a' parameter
use std::fmt::Display;

// Define the function `longest_with_an_announcement` which accepts two string slices (`x` and `y`),
// and an additional parameter `ann` of a generic type `T`. The lifetime `'a` ensures that the 
// references `x` and `y` live at least as long as the function call.
fn longest_with_an_announcement<'a, T>(
    x: &'a str, // First string slice with lifetime 'a
    y: &'a str, // Second string slice with lifetime 'a
    ann: T,       // Generic parameter `ann` of type `T` which implements the Display trait
) -> &'a str  // returns a reference to the longer of the two string slices, with lifetime 'a
where
    T: Display, // `T` must implement the Display trait, allowing `ann` to be printed
{
    // Print the announcement using the Display trait for type `T`
    println!("Announcement! {ann}");

    // Return the longer of the two string slices
    if x.len() > y.len() {
        x // Return `x` if it is longer
    } else {
        y // Return `y` otherwise
    }
}

fn main() {

    println!(r"
       _                              _ _  __        _ 
      | |                            | (_)/ _|      | |
   ___| |__   ___   ___  ___  ___    | |_| |_ ___   | |
  / __| '_ \ / _ \ / _ \/ __|/ _ \   | | |  _/ _ \  | |
 | (__| | | | (_) | (_) \__ \  __/   | | | ||  __/  |_|
  \___|_| |_|\___/ \___/|___/\___|   |_|_|_| \___|  (_)
");

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string3 = String::from("long string is long");

    {
        let string4 = String::from("xyz");
        let result = longest(string3.as_str(), string4.as_str()); // result has the same lifetime as string4, which is the `y: &'a str` parameter in longest()
        println!("The longest string is {result}");
    }

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    
    // An instance of ImportantExcerpt can’t outlive the reference it holds in its part field.
    let i = ImportantExcerpt {
        part: first_sentence,
    };

    // 'static, which denotes that the affected reference can live for the entire duration of the program.
    let s: &'static str = "I have a static lifetime";

}
