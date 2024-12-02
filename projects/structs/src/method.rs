/*
Methods are similar to functions: we declare them with the fn keyword and a name,
they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// To define the function within the context of Rectangle, we start an impl (implementation) block for Rectangle. Everything within this impl block will be associated with the Rectangle type.
impl Rectangle {
    fn area(&self) -> u32 { // The &self is actually short for self: &Self.
        return self.width * self.height
    }

    fn width_bool(&self) -> bool {
        return self.width > 0
    }
}

pub fn main() {
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect3 = Rectangle {
        width: 0.5,
        height: 50
    }

    println!("The area of rect3 is {} square pixels.", rect3.area());
}