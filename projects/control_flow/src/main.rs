
// Because if is an expression
// we can use it on the right side of a let statement to assign the outcome to a variable.
fn if_let() {
    let first_condition = true;
    let second_condition = 'Z';
    let number = if first_condition { 5 } else { 6 };
    let character = if second_condition == 'Z' {
                        true
                    }
                    else {
                        false
                    };

    println!("The 1st number is: {}", number);
    println!("Is the character, 'Z'?: {}", character);
}

fn is_even(value: i32) -> bool { // explicit type annotation of 32 bit integer as argument, returns a boolean value
    if value % 2 == 0 {
        return true;
    } 
    else {
        return false;
    }
}

fn main() {
    if_let();

    let first = is_even(16);
    println!("Is 16 even? {}", first);
    
    let second = is_even(13);
    println!("Is 13 even? {}", second);

}
