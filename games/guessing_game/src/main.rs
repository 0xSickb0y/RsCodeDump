use std::io;
use std::cmp::Ordering;
use rand::Rng;

pub struct Guess {
    value: i32,
}

// Functions can use the Guess type in their signature and confidently use the values they recieve
// A Guess type will only be created if the new() function receives a value between 1 and 100.
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            eprintln!("Guess value must be between 1 and 100, got {value}.\n");
        }
        return Guess { value } // Creates a new Guess with its value field set to the value parameter of ::new()
    }
    
    // "getter" method, get's data from it's field and return it.
    // It's important that the value field be private so code using the Guess struct is not allowed to set the value directly.
    // thereby ensuring there’s no way for a Guess to have a value that hasn’t been checked by the conditions in the Guess::new function.
    pub fn value(&self) -> i32 { 
        return self.value
    }
}

fn main() {
    println!("Guess the number!");
    
    // .thread_rng() gives us the generator we’re going to use, local to the current thread, seeded by the OS.
    // .gen_range() method defined by rand::Rng | current range: start..=end < INCLUSIVE
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut buf = String::new();
    
        io::stdin()
            .read_line(&mut buf)
            .expect("Failed to read line");
        
        // .trim() strips ' ' and '\n' or'\r\n'
        // .parse() converts a string to another type, the ":" character anotates the type for .parse()
        // The u32 annotation and the comparison with secret_number means Rust will infer that secret_number should be u32 as well.
        let value: i32 = match buf.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // "_" char == catchall value (case: all errors), then continues to the next iteration of the loop
            // Effectively, the program ignores all errors that parse might encounter.           
        };

        let guess = Guess::new(value);
        match guess.value().cmp(&secret_number) { // Receive return value from .cmp() method
            // handle the return value properly
            Ordering::Less => println!("Less!"),
            Ordering::Greater => println!("Greater!"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }        
    }
}
