use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    // .thread_rng() gives us the generator we’re going to use, local to the current thread, seeded by the OS.
    // .gen_range() method defined by rand::Rng | current range: start..=end < INCLUSIVE
    let secret_number = rand::thread_rng().gen_range(1..=100);
    
    loop {
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        // second "guess" variable shadows the first one
        // .trim() strips ' ' and '\n' or'\r\n'
        // .parse() converts a string to another type, the ":" character anotates the type for .parse()
        // The u32 annotation and the comparison with secret_number means Rust will infer that secret_number should be u32 as well.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // "_" char == catchall value (case: all errors), then continues to the next iteration of the loop
            // Effectively, the program ignores all errors that parse might encounter.           
        };
    
        // Receive return value from .cmp() method
        match guess.cmp(&secret_number) {
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
