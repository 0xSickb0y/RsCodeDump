/* 
So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate file to make the code easier to navigate.
*/

mod front_of_house; // Note that you only need to load a file using a mod declaration once in your module tree.

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}