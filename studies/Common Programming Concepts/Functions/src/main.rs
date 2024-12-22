
// don't name return values, declare their type with `->`
fn return_value(value: i32) -> i32{
    // The return value is synonymous with the value of the final expression in the body.
    // You can also return early from a function by using `return`
    println!("[RETURN VALUE]");
    return value + 1 // or simply: value + 1
}

fn statements_and_expressions() {
    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resultant value. 
    
    println!("\n[STATEMENTS AND EXPRESSIONS]");

    let x = 5; // statement
    let y = { // expression
        let z = 1;
        z + x // Expressions do not include ending semicolons.
    }; // If you add a semicolon to the end of an expression, you turn it into a statement.

    println!("Statement value: {}\nExpression result: {}", x, y)

}


// creates a function with two arguments, 32 bit integer and a character
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("\n[PRINT LABELED MEASUREMENT]");
    println!("The value is: {}{}", value, unit_label);    
}

// declare function, receive 32 bit integer as argument with explicit type annotation
fn another_function(x: i32) {

    println!("\n[ANOTHER FUNCTION]");
    println!("The value of x is: {}", x);

    // In function signatures, you must declare the type of each parameter.
    // This is a deliberate decision in Rust’s design:
    // requiring type annotations in function definitions means the compiler 
    // almost never needs you to use them elsewhere in the code to figure out what type you mean.

}

fn main() {
    let six = return_value(5);
    println!("The return value is: {}", six);

    statements_and_expressions();
    
    print_labeled_measurement(9001, 'h');
    
    another_function(1337);
    
    println!("Main: Hello, world!");
}