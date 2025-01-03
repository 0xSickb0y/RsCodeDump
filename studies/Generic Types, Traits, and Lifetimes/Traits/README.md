# Traits: Defining Shared Behavior

A _trait_ defines the functionality a particular type has and can share with other types. We can use traits to define shared behavior in an abstract way.

We can use _trait bounds_ to specify that a generic type can be any type that has certain behavior.

> Traits are similar to a feature often called interfaces in other languages, although with some differences.

A type's behavior consists of the methods we can call on that type. Different types share the same behavior if we can call the same methods on all of those types.
Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

## Implementing Traits for Local Types

In Rust, you can implement a trait for a type as long as either the trait or the type, or both, are local to your crate. This allows you to extend functionality for types defined within your crate. For example, if you define a custom type, you can implement a trait on it to provide specific behavior relevant to your type.

You can also implement traits from the standard library or other crates on types local to your crate. This allows you to extend the functionality of those types by implementing methods for them, such as formatting or computation methods.

## The Orphan Rule and Coherence

Rust enforces a restriction called the Orphan Rule when implementing external traits for types. The Orphan Rule states that you cannot implement external traits for types that are also defined externally (i.e., types that come from the standard library or other third-party crates).

The reason for this rule is __coherence__ a property that ensures that there is only one implementation of a trait for a given type in a given context. Without the Orphan Rule, multiple crates could attempt to implement the same trait for the same type, leading to ambiguity about which implementation should be used.

## Blanket Implementations

We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are used extensively in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait. This means that as long as a type implements Display, it automatically has the ToString functionality available, without requiring an explicit ToString implementation for that type.

## Conclusion

Traits and trait bounds let us write code that uses generic type parameters to reduce duplication but also specify to the compiler that we want the generic type to have particular behavior. The compiler can then use the trait bound information to check that all the concrete types used with our code provide the correct behavior.