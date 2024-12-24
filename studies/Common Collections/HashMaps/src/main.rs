#![allow(dead_code, unused)]
use std::collections::HashMap; // Import HashMap from the collections portion of the standard library.
                              // Hash maps have less support from the standard library; there’s no built-in macro to construct them.
                             // Hash maps store their data on the heap, they are also homogeneous
                            // All of the keys must have the same type, and all values must have the same type
                           // Each key can only have one value associated with it at a time, but not vice versa, that is: Red team and Blue team can have the value 10 in their score.
fn main() {
    let mut scores = HashMap::new(); // Creates HashMap<String, i32>

    // Adds elements with .insert(k, v)
    scores.insert(String::from("Red"), 10); 
    scores.insert(String::from("Blue"), 15);

    // Accessing Values in a Hash Map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name) // We can get a value out of the hash map by providing its key to the .get() method. Returns Option<&V>
        .copied() // Handles the Option by calling .copied() to get an Option<i32> rather than Option<&i32>
        .unwrap_or(0); // Set score to 0 if scores doesn't have an entry for the key.

    // Iterating over key/values in a Hash map
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.
    let mut map0 = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map0.insert(field_name, field_value); // field_name and field_value are invalid after this point
    for (key, value) in &map0 {
        println!("Map0 = {key}: {value}");
    }

    // If we insert references to values into the hash map, the values won’t be moved into the hash map.
    // The values that the references point to must be valid for at least as long as the hash map is valid. 
    let mut map1 = HashMap::new();
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    map1.insert(&field_name, &field_value);
    for (key, value) in &map1 {
        println!("Map1 = {key}: {value}");
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 17);
    println!("{scores:?}"); // The original value of 10 has been overwritten.

    // Adding a Key and Value Only If a Key Isn’t Present
    // Hash maps have a special API for this called entry that takes the key you want to check as a parameter.
    scores.entry(String::from("Yellow")).or_insert(50); // .or_insert() method returns a mutable reference to the value for the corresponding entry key if the key exists.
    println!("{scores:?}");

    // Updating the value based on the Old value
    let mut map = HashMap::new();
    let text = "hello world wonderful world";

    for word in text.split_whitespace() { // Returns an iterator over subslices, separated by whitespace, of the value in text.
        let count = map.entry(word).or_insert(0); // Returns a mutable reference to the value for the specified key
        *count += 1; // Dereference `count`
    }
    // Mutable reference goes out of scope
    println!("{map:?}");
}
