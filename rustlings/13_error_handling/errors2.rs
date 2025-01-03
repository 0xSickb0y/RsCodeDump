// Say we're writing a game where you can buy items with tokens. All items cost
// 5 tokens, and whenever you purchase items there is a processing fee of 1
// token. A player of the game will type in how many items they want to buy, and
// the `total_cost` function will calculate the total cost of the items. Since
// the player typed in the quantity, we get it as a string. They might have
// typed anything, not just numbers!
//
// Right now, this function isn't handling the error case at all. What we want
// to do is: If we call the `total_cost` function on a string that is not a
// number, that function will return a `ParseIntError`. In that case, we want to
// immediately return that error from our function and not try to multiply and
// add.
//
// There are at least two ways to implement this that are both correct. But one
// is a lot shorter!

use std::num::ParseIntError;

fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;

    // Try to parse the input string into an integer
    let qty = item_quantity.parse::<i32>()?;

    let total_cost = (cost_per_item * qty) + processing_fee;

    return Ok(total_cost)
}


fn main() {
    // You can optionally experiment here.
    let result = total_cost("34").unwrap();
    println!("{result}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::IntErrorKind;

    #[test]
    fn item_quantity_is_a_valid_number() {
        assert_eq!(total_cost("34"), Ok(171));
    }

    #[test]
    fn item_quantity_is_an_invalid_number() {
        assert_eq!(
            total_cost("beep boop").unwrap_err().kind(),
            &IntErrorKind::InvalidDigit,
        );
    }
}