#![allow(dead_code)]
/*
By using modules, we can group related definitions together and name why they’re related.
Programmers using this code can navigate the code based on the groups rather than having to read through all the definitions,
making it easier to find the definitions relevant to them. 
*/

mod front_of_house {
    pub mod hosting { // `hosting` and `serving` are siblings defined in the same module. Both are child modules of `front_of_house`
        pub fn add_to_waitlist() {} // Modules can also hold definitions for other items, such as structs, enums, constants, traits, and functions.
        pub fn seat_at_table() {}
    }

    pub mod serving { // `hosting` and `serving` are siblings defined in the same module. Both are child modules of `front_of_house`
        pub fn take_order() {}
        pub fn serve_order() {}
        pub fn take_payment() {}
    }
}

mod customer {
    // Create shortcut to a path using the `use` keyword
    // Paths brought into scope with `use` also check privacy, like any other paths.
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant_example() {
        // We could also achieve the same thing by using:
        // super::front_of_house::hosting::add_to_waitlist();
        hosting::add_to_waitlist();
    }
}


mod back_of_house {
    pub struct Breakfast { // If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private.
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        // because back_of_house::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast.
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    
    // If we make an enum public, all of its variants are then public.
    #[derive(Debug)]
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using `super`` at the start of the path.
        /* 
        We think the back_of_house module and the deliver_order function are likely to stay in the same relationship to each other
        and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used super so we’ll have fewer places to update code in the future if this code gets moved to a different module.
        */
    }

    fn cook_order() {}
}





pub fn deliver_order() {}