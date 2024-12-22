/*
When compiling a crate, the compiler starts from the crate root (typically `src/lib.rs` for libraries or `src/main.rs` for binaries).
Modules are declared with `mod` and can be defined inline, in a file like `src/<module>.rs`, or in `src/<module>/mod.rs`.

Submodules are declared within parent modules and are looked for in corresponding subdirectories with similar naming conventions.
Once a module is part of your crate, it can be accessed via paths like `crate::garden::vegetables::Asparagus`.

By default, code in modules is private, but using `pub` before `mod` or item declarations makes them public.
The `use` keyword creates shortcuts for long paths, allowing you to refer to items without repeating the full path each time.
*/

use crate::garden::vegetables::Asparagus;

pub mod garden; // Tells the compiler to include the code in `src/garden.rs`

fn main() {
    let asparagus = Asparagus {
        amount: 55,
    };

    println!("Asparagus:\n");
    println!("Ammount: {}", asparagus.amount);
}
