#![allow(dead_code)]

/*
When bringing a function into scope with use, it's idiomatic to import the parent module,
which minimizes repetition of the full path while still making it clear where the function is defined. This approach ensures that the function's origin is explicit.

On the other hand, omitting the parent module makes the function’s definition unclear, which is considered less idiomatic.
For structs, enums, and other items, it's more common to specify the full path in the use statement. This makes the origin of the imported item unambiguous and easy to identify.

The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.
*/
use std::io;
use std::fmt;

// If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator:
use std::collections::*;

/*
There’s another solution to the problem of bringing two types of the same name into the same scope with `use`:
after the path, we can specify `as` and a new local name, or alias, for the type.

When we bring a name into scope with the use keyword, the name available in the new scope is private.
To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
*/
pub use std::io::Result as IoResult;


/*
As you can see, using the parent modules distinguishes the two Result types.

If instead we specified use std::fmt::Result and use std::io::Result, we’d have two Result types in the same scope,
and Rust wouldn’t know which one we meant when we used Result.
*/
fn function1() -> fmt::Result {
    return Err(fmt::Error)
}

fn function2() -> io::Result<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    return Ok(buffer)
}

fn function3() -> IoResult<String> {
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer)?;

    return Ok(buffer)
}

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);

    
}
