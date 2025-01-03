#![allow(unused)]
// We can also define structs to use a generic type parameter in 
// one or more fields using the <> syntax.

struct Point<T> { // Hold x and y coordinate values of any type T.
    x: T,
    y: T,
}

struct PointDiff<T, U> { // Hold x and y coordinate values of any type T and U.
    x: T,
    y: U,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let int_float = PointDiff { x: 5, y: 4.0};

}
