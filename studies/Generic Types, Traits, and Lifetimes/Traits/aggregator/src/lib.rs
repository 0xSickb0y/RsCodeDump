use std::fmt::{Debug, Display};

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author()) // Default behavior for summarize()
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("By {}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool, 
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


pub fn notify(item: &impl Summary) { // &impl Summary: some type that implements the Summary trait.
    println!("Breaking news! {}", item.summarize());
}

// same declaration with trait bound:
pub fn notify_trait_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize()); 
}

// Multiple bounds with the + syntax
pub fn notify_mult_trait_bound<T: Summary + Display>(item: &T) {
    let result = format!("Breaking news! {}", item.summarize());
    println!("{result}");
}


// Trait bounds with `where` clauses
pub fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone,
    U: Clone + Debug,
{
    let cloned_t = t.clone();
    let cloned_u = u.clone();

    println!("{}", cloned_t);
    println!("{:?}", cloned_u);

    42
}

// return a value of some type that implements the Summary trait
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people.",
        ),
        reply: false,
        retweet: false,
    }
}

// Define a generic Pair struct that holds two values of type T
pub struct Pair<T> {
    x: T,
    y: T,
}

// Implement methods for Pair<T> where T can be any type
impl<T> Pair<T> {
    // The 'new' method creates a new instance of Pair<T> with the given values x and y
    pub fn new(x: T, y: T) -> Self {
        Self { x , y }
    }
}

// Implement additional methods for Pair<T> only when T satisfies both the Display and PartialOrd traits
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest number is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }
}
