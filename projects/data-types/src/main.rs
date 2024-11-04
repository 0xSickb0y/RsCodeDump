
fn scalar_types() {

    println!("\n[SCALAR TYPES]\n");

    // integers
    let x = 4; // i32 signed
    let y: i8 = 5; // signed
    let z: u8 = 6; // unsigned

    println!("i32: {}\ni8: {}\nu8: {}\n", x, y , z);    

    // float
    let a = 7.5; // floating 64 bit
    let b: f32 = 8.1;  // floating 32 bit

    println!("f64: {}\nf32: {}\n", a, b);

    // booleans

    let t = true;
    let f: bool = false; // explicit type annotation

    println!("Bool: {}\nBool: {}\n", t, f);

    // characters
    let d = 'z'; // '' char literal 4 bytes Unicode Scalar Value | "" string literal
    let z: char = 'ℤ'; // explicit type annotation
    let crab = '🦀';

    println!("Char Literal: {}\nChar Unicode: {}\nEmoji: {}", d, z, crab);

}

fn compound_types() {

    println!("\n[COUMPOUND TYPES]\n");

    // tuples
    let tup_one: (i8, i16, i32) = (6, 32, 256); // create a tuple with explicit type annotation
    let (x, y ,z) = tup_one; // use a pattern with `let` for destructuring

    let tup_two: (char, i32, bool) = ('a', 256, false); // create a tuple with explicit type annotation
    let a = tup_two.0; // access each element of the tuple by their respective indices
    let b = tup_two.1;
    let c = tup_two.2;

    println!("Tuple 1: {}, {}, {}", x,  y, z);
    println!("Tuple 2: {}, {}, {}\n", a , b, c);

    // arrays
    let arr_one = [10, 20, 30, 40, 50]; // must have same type and fixed length
    let arr_two: [i32; 3] = [100, 200, 300]; // explicit type annotation
    let arr_three = [3; 5]; // initialize the array with same value for each element
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    println!("Array one [4]: {}\nArray two [1]: {}\nArray three [0]: {}\n2nd Month: {}", 
              arr_one[4], arr_two[1], arr_three[0], months[1]);

}

fn numeric_operations() {

    println!("\n[NUMERIC OPERATIONS]\n");
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    println!("sum: {}\ndifference: {}\nproduct: {}\nquotient: {}\ntruncated: {}\nreaminder: {}\n",
              sum, difference, product, quotient, truncated, remainder);
}

fn main() {

    scalar_types();
    compound_types();
    numeric_operations();

}
