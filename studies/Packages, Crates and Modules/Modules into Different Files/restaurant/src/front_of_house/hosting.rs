/*
If we instead put hosting.rs in the src directory, the compiler would expect the hosting.rs code
to be in a hosting module declared in the crate root, and not declared as a child of the front_of_house module. 
*/
pub fn add_to_waitlist() {}