#![allow(dead_code, unused)]

/*
When a function’s implementation calls something that might fail, instead of handling the error within the function itself
you can return the error to the calling code so that it can decide what to do.

This is known as propagating the error and gives more control to the calling code,
where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.
*/

use std::{
    env, process,
    fs::File,
    error::Error,
    io::{self, Read},
};

// If we don't have enough information on what the calling code is trying to do
// We propagate all the success or error information upward for it to handle appropriately.
fn read_uname_from_file(file_name: &str) -> Result<String, io::Error> {
    let file_result = File::open(file_name);
    let mut file_handle = match file_result {
        Ok(file) => file,
        Err(e) => return Err(e) // Return early out of the function entirely and pass the error value from File::open(), in this case: `e`
    };

    let mut uname = String::new();
    let result = match file_handle.read_to_string(&mut uname) { // Reads all bytes until EOF in this source, appending them to buf.
        Ok(_) => Ok(uname),                                    // We need a match expression here because .read_to_string() might fail even though File::open() succeeded 
        Err(e) => Err(e)
    };
    return result;
}

// Has the same functionality as the previous function but uses the ? operator
// Error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library
// When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.
fn read_uname_from_file_alt(file_name: &str) -> Result<String, io::Error>{
    let mut file_result = File::open(file_name)?; // If Ok(): Returns the value inside the Ok, and continue.
                                                 // If Err(): Returns an error from the whole function, propagates the rrror to the calling code
    let mut uname = String::new();
    file_result.read_to_string(&mut uname)?; // Same thing applies here
    
    return Ok(uname)
}

// ? can also be used with Option<T>. You can only use it in a function that returns an Option.
fn option_example(text: &str) -> Option<char> {
    return text.lines() // Returns an iterator of the lines in the &str
        .next()? // Get the first value from the iterator 
                // If the value is None: None will be returned from the function
               // If the value is Some: The value inside Some is the resultant value of the expression, and the function continues.
        .chars() // Returns an iterator over the chars of a string slice
        .last() // Last item in the iterator
                                              
}

// The main() function can also return a `Result<(), E>`
// The main function may return any types that implement the std::process::Termination trait
fn main() -> Result<(), Box<dyn Error>> { // Box<dyn error> is a trait object. For now, you can read Box<dyn Error> to mean “any kind of error.”
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage:");
        eprintln!("cargo run FILE_NAME");
        process::exit(0);
    }

    let file_name = args[1].as_str();
    let result = read_uname_from_file(file_name);
    // Handles either Ok or Err from the read_uname_from_file() function
    match result {
        Ok(_) => println!("read_uname_from_file(): Success"),
        Err(e) => panic!("read_uname_from_file(): Failed\nReason: {e}")
    };

    let greeting_file = File::open("does_not_exits")?; // Using ? on a Result in main() with Box<dyn Error>
                                                      // Is allowed because it allows any Err value to be returned early.
                                                     // Even though this function will only return io::Error
                                                    // by specifying Box<dyn Error>, this signature will continue to be correct
                                                   // even if more code that returns other errors is added to the body of main.  
    return Ok(())
}
