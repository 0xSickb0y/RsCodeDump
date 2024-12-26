/*
There are two ways to cause a panic in practice: by taking an action that causes your code to panic.
or by explicitly calling the `panic!()` macro.

By default these panics will print a failure message, unwind, clean up the stack, and quit.
You can have Rust display the call stack with the RUST_BACKTRACE=1 | RUST_BACKTRACE=full environment variables.

Rust also allows us to immediately abort the program by setting panic = "abort" on the Cargo.toml file.
Memory that the program was using will then need to be cleaned up by the OS. 
*/

fn main() {
    panic!("Crash and burn."); // The panic! call might be in code that our code calls,
                              // and the filename:line reported by the error message
                             // will be someone else's code where the `panic!` macro is called,
                            // Not the line of our code that eventually led to the `panic!` call.
}
