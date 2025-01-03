// We can implement methods on structs and enums and use generic types in their definitions.
struct Point<T> {
    x: T,
    y: T,
}

// Declare <T> after `impl` so we can use T to specify that we're implementing methods on the type `Point<T>`
impl<T> Point<T> {
    fn x(&self) -> &T { // Method x returns a reference to the data in the x field.
        &self.x
    }
}

// Here we use the concrete type f32, meaning we don’t declare any types after impl.
impl Point<f32> {
    // measures how far our point is from the point at coordinates (0.0, 0.0) 
    fn distance_from_origin(&self) -> f32 { // Other instances of Point<T> where T is not of type f32 will not have this method defined.
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Define a struct `PointMix` with generic types `X1` and `Y1` for its fields.
struct PointMix<X1, Y1> {
    x: X1,
    y: Y1,
}

// Implement methods for the `PointMix` struct with generic types `X1` and `Y1`.
// The purpose of this example is to demonstrate a situation in which some generic parameters are declared with impl and some are declared with the method definition. 
impl<X1, Y1> PointMix<X1, Y1> { //  X1 and Y1 are declared after impl because they go with the struct definition.
    fn mixup<X2, Y2>(self, other: PointMix<X2, Y2>) -> PointMix<X1, Y2> { // X2 and Y2 are declared after fn mixup because they’re only relevant to the method.
        PointMix { x: self.x, y: other.y }
    }
}

fn main() {
    let p = Point { x: 5, y: 10};
    let fp: Point<f32> = Point { x: 13.4, y: 34.2};

    println!("p.x = {}", p.x());
    println!("Distance from origin: {}", fp.distance_from_origin());

    let p1 = PointMix { x: 5, y: 10.4 };
    let p2 = PointMix { x: "hello", y: 'c' };
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
