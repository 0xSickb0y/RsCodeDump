// We use generics to create definitions for items like function signatures or structs,
// which we can then use with many different concrete data types.

// The following two functions both find the largest value in a slice,
// and can be combined into a single function that uses generics (fn largest<T: std::cmp::PartialOrd>)
fn largest_i32(list: &[i32]) -> &i32 {
    println!("largest_i32(list: &[i32])");

    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

fn largest_char(list: &[char]) -> &char {
    println!("largest_char(list: &[char])");

    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    };

    largest
}

// We read this definition as: the function `largest()` is generic over some type T.
// This function has one parameter named list, which is a slice of values of type T.
// The `largest()` function will return a reference to a value of the same type T.
// Implement trait: `std::cmp::PartialOrd`, restrict types valid for T to only those
// that implement `PartialOrd` 
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    println!("largest<T: std::cmp::PartialOrd>(list: &[T])");
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let num_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    println!("The largest number is {}\n", largest_i32(&num_list));
    println!("The largest char is {}\n", largest_char(&char_list));

    println!("The largest number is {}\n", largest(&num_list));
    println!("The largest char is {}\n", largest(&char_list));
    
}
