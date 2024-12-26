//  let’s see what it’s like when a panic! call comes from a library because of a bug in our code instead of from our code calling the macro directly.
// run with: RUST_BACKTRACE=1 cargo run

fn main() {
    let v = vec![0, 1 ,2];

    v[99]; // [] Returns an element, but 99 is out of bounds.

    /*
    thread 'main' panicked at src/main.rs:7:6:
    index out of bounds: the len is 3 but the index is 99
    stack backtrace:
    0: rust_begin_unwind
                at /rustc/--snip--/library/std/src/panicking.rs:662:5
                
    1: core::panicking::panic_fmt
                at /rustc/--snip--/library/core/src/panicking.rs:74:14
                
    2: core::panicking::panic_bounds_check
                at /rustc/--snip--/library/core/src/panicking.rs:276:5
                
    3: <usize as core::slice::index::SliceIndex<[T]>>::index
                at /rustc/--snip--/library/core/src/slice/index.rs:302:10
                
    4: core::slice::index::<impl core::ops::index::Index<I> for [T]>::index
                at /rustc/--snip--/library/core/src/slice/index.rs:16:9
                
    5: <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index
                at /rustc/--snip--/library/alloc/src/vec/mod.rs:2920:9
                
    6: index_out_of_bounds::main
                at ./src/main.rs:7:6
                
    7: core::ops::function::FnOnce::call_once
                at /rustc/--snip--/library/core/src/ops/function.rs:250:5
    */

    // If we don’t want our program to panic, we should start our investigation at the location pointed to by the first line mentioning a file we wrote.
    // In this case: index_out_of_bounds::main at ./src/main.rs:7:6
}
