#![allow(dead_code, unused)]

use std::{
    env,
    process,
    fs::File,
    io::ErrorKind
};

fn print_usage_exit() {
    eprintln!("Usage:\n");

    eprintln!("cargo run 0 -> if_else_closure_handle()");
    eprintln!("cargo run 1 -> match_file_handle_all_err()");
    eprintln!("cargo run 2 -> match_file_handle_diff_err()");
    eprintln!("cargo run 3 -> unwrap_method()");
    eprintln!("cargo run 4 -> expect_method()");

    process::exit(0);
}

// Handle the logic using closures and .unwrap_or_else
fn if_else_closure_handle() {
    let file_name = "hello.txt";
    let result = File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Failed to create file: {file_name}\n{error:?}");
            })
        } else {
            panic!("Failed to open file: {file_name}\n{error:?}");
        }
    });

}

// Panics no matter why File::open failed. 
fn match_file_handle_all_err() {
    let file_name = "hello.txt";
    let result = File::open(file_name); // ::open() returns Result<T, E>
                                       // This call might succeed and return a file handle or it can fail and return an Err
    
    let file_handle = match result { //
        Ok(file) => file, // Returns the inner `file` value out of the `Ok` variant, then assign that file handle value to the variable `file_handle`
                         // After that, we can use the handle to read/write to the file
        Err(error) => panic!("Failed to open file: {file_name}\n{error:?}"),
    };
}

// Properly handle the `no such file` error with a match expression to create the file.
fn match_file_handle_diff_err() {
    let file_name = "hello.txt";
    let result = File::open(file_name);

    let file_handle = match result {
        Ok(file) => file,
        Err(error) => match error.kind() { // Returns io::Error, struct provided by the standard library.
                                          // .kind() method returns an `io::ErrorKind` value
            ErrorKind::NotFound => match File::create(file_name) { // Create the file if it doesn't exist.
                Ok(file_create) => file_create,
                Err(error) => panic!("Failed to create file: {file_name}\n{error:?}"),
            },
            other_err => {
                panic!("Failed to open file: {file_name}\n{error:?}"); // Panic on all errors that are not ErrorKind::NotFound
            }
        }
    };
}

fn unwrap_method() {
    let file_name = "hello.txt";
    let result = File::open(file_name).unwrap(); // Shortcut method implemented like the match expression
                                                // If Result == Ok(): returns returns the value inside Ok
                                               // If Result == Err(): calls the panic! macro
}

fn expect_method() {
    let file_name = "hello.txt";
    let result = File::open(file_name)
        .expect(".expect() message: Failed to open file"); // The expect method lets us also choose the panic! error message.
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage_exit();
    }

    match args[1].as_str() {
        "0" => {
            println!("Running example: if_else_closure_handle()");
            if_else_closure_handle();
        },
        "1" => {
            println!("Running example: match_file_handle_all_err()");
            match_file_handle_all_err();
        },
        "2" => {
            println!("Running example: match_file_handle_diff_err()");
            match_file_handle_diff_err();
        },
        "3" => {
            println!("Running example: unwrap_method()");
            unwrap_method();
        },
        "4" => {
            println!("Running example: expect_method()");
            expect_method();
        }
        other => {
            eprintln!("Invalid option: {other}");
            print_usage_exit();
        }
    }
}
