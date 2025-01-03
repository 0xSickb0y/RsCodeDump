# Validating References with Lifetimes

Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be. Every reference in Rust has a _lifetime_, which is the scope for which that reference is valid. Most of the time lifetimes are implicit and inferred, just like most of the time, types are inferred.

We must annotate types only when multiple types are possible. In a similar way, we must annotate lifetimes when the lifetimes or references could be related in a few different ways. Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

## Dangling References

The primary goal of lifetimes in Rust is to prevent dangling references, which occur when a reference points to memory that is no longer valid. This situation leads to undefined behavior, as the program might try to access data that no longer exists or has been deallocated. For example, consider a program with an outer scope and an inner scope. A reference is made to a variable within the inner scope, but when the inner scope ends, the variable goes out of scope, making the reference invalid.

Rust’s lifetime rules prevent this by ensuring that references are always valid for as long as they are used. In this case, if we attempt to use a reference to a variable that has already gone out of scope, Rust will generate a compile-time error. This ensures that no reference can outlive the data it points to, thereby avoiding the risk of dangling references. 

```rust
fn lifetime_bug() { // visual representation of the lifetimes of the variables x and r
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |     -> `x` does not live long enough
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          | // subject of the reference doesn’t live as long as the reference.
}  
```

## Generic Lifetimes in Functions

In Rust, when writing functions that take references as parameters and return a reference, the compiler cannot always infer the relationship between the lifetimes of the input references and the returned reference. Specifically, when the function involves conditional logic—such as returning a reference to one of the input parameters based on a comparison—it’s unclear whether the returned reference will be valid for the same duration as one input, the other, or both.

In such cases, the Rust compiler requires us to explicitly annotate the lifetimes using generic lifetime parameters. These parameters tell the compiler how long the returned reference will be valid, based on the lifetimes of the inputs. 

## Lifetime Elision 

In Rust, the lifetime elision rules allow the compiler to automatically infer the lifetimes of references in certain situations, making code less verbose and more readable. These rules were introduced to handle common, predictable patterns, reducing the need for explicit lifetime annotations in function signatures. The rules apply in specific cases where the relationship between the lifetimes is clear and unambiguous, so the programmer doesn’t have to manually annotate them.

The first rule states that each reference parameter gets its own lifetime. For example, in a function that takes two references as parameters, the compiler will automatically infer two distinct lifetimes for those parameters.

The second rule applies when there is exactly one reference parameter: the output reference’s lifetime is inferred to be the same as the input reference’s lifetime. This means if a function takes a reference and returns a reference, the compiler assumes the returned reference’s lifetime is tied to that of the input.

Lastly, the third rule applies to methods: if one of the input parameters is &self or &mut self, the compiler assigns the lifetime of self to all output references. This makes methods much more concise, as the lifetime of self implicitly applies to the method's return value without further annotation.

These elision rules simplify common cases, but they don't provide full inference. If the compiler encounters ambiguity after applying these rules, it will fail to compile and prompt you to add the necessary lifetime annotations.

## The Static Lifetime

`'static` denotes that the affected reference can live for the entire duration of the program. 

```rust
let s: &'static str = "I have a static lifetime.";
```

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is `'static`.

You might see suggestions to use the 'static lifetime in error messages. But before specifying 'static as the lifetime for a reference, think about whether the reference you have actually lives the entire lifetime of your program or not, and whether you want it to. Most of the time, an error message suggesting the 'static lifetime results from attempting to create a dangling reference or a mismatch of the available lifetimes. In such cases, the solution is to fix those problems, not to specify the 'static lifetime.