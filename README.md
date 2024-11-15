# RsCodeDump

Personal repository dedicated to organizing RUST source code, experiments, and cheat sheets. 

![image](https://github.com/user-attachments/assets/1b5365eb-8920-4719-90ab-37bab57a29dc)

### Structure

Just like other languages, Rust starts with the `main()` function. To declare a function in Rust, we use `fn FUNCTION_NAME {}`

```rust
fn main() {
    // code here
}
```

### Stdout

To print something to the standard output, we use `println!()`

```rust
fn main() {
    println!("Hello, world!");
}
```

### Assembly

Analyzing our program instructions in assembly

![Pasted image 20241031052651](https://github.com/user-attachments/assets/91af6e7d-f936-4c62-bd99-56f953dd4328)

1. **Function Prologue**:
   ```asm
   0x55555555aa40 <+0>:  subq   $0x38, %rsp
   ```
   - Allocates space on the stack for local variables and function calls.

2. **Prepare Argument for `println!`**:
   ```asm
   0x55555555aa44 <+4>:  leaq   0x8(%rsp), %rdi
   ```
   - Loads the address for the first argument, which is a pointer to the string "Hello, world!". The `println!` macro formats this string for output.

3. **Load String Address**:
   ```asm
   0x55555555aa49 <+9>:  leaq   0x4db20(%rip), %rsi
   ```
   - This instruction loads the address of the "Hello, world!" string into `%rsi`. The `(%rip)` addressing mode allows the program to access the string data in a position-independent manner.

4. **Call Formatting Function**:
   ```asm
   0x55555555aa50 <+16>: callq  0x55555555aa00
   ```
   - Calls a function (likely related to formatting the string) that prepares the arguments for `println!`.

5. **Prepare for the Output Call**:
   ```asm
   0x55555555aa55 <+21>: leaq   0x8(%rsp), %rdi
   ```
   - Again prepares an argument, this time possibly for the actual output function that will handle printing.

6. **Indirect Call**:
   ```asm
   0x55555555aa5a <+26>: callq  *0x50440(%rip)
   ```
   - Calls the final output function (e.g., to print to the console).

7. **Function Epilogue**:
   ```asm
   0x55555555aa60 <+32>: addq   $0x38, %rsp
   ```
   - Cleans up the stack by restoring the stack pointer.

8. **Return from Function**:
   ```asm
   0x55555555aa64 <+36>: retq
   ```
   - Returns from the `main` function.

The assembly code generated from the program demonstrates how Rust handles the `println!` macro. 

- **Stack Management**: The code manages stack space efficiently, allocating and deallocating memory as needed for function calls and local variables.
  
- **Argument Preparation**: It prepares the necessary arguments for the `println!` macro, loading string addresses into registers.

- **Function Calls**: The assembly code includes calls to functions that manage string formatting and output, showcasing how Rust abstracts these operations into simple macros.

### Types and Variables

> https://doc.rust-lang.org/std/all.html#primitives

The word `let` declares a variable just like in JavaScript. You can also define a type for the variable.

By default, variables in Rust are immutable (not to be confused with constants). To declare mutability in a variable, we use `mut`: 

```rust
let mut variable: bool = true; 
```

In this example, we declared an 8-bit integer and assigned it the number 64 (the maximum value for `i8`, which can represent values from -128 to 127). For unsigned integers, we use `u8`, which has a range from 0 to 255.

The smallest value that can be used in an unsigned integer is 0, meaning it cannot contain negative values.

Finally, we used string interpolation with `{}` brackets to print the variables to stdout. Here’s an example:

```rust
let name = "Alice";
println!("Hello, {}!", name);
```

![Pasted image 20241031202835](https://github.com/user-attachments/assets/0632d8fd-8e70-43f3-b7a5-5fe33ca7b383)

To print the size of any variable, we can use `std::mem::size_of_val(&first)`.

![Pasted image 20241031202849](https://github.com/user-attachments/assets/460a6653-85d9-4916-9186-f4fac3acbb8c)

If we try to declare a `char` using double quotes, we get an error: expected `char`, found `&str`.

Help: If you meant to write a `char` literal, use single quotes. (rustc --explain E0308)

In Rust, a `char` is 4 bytes because it represents a Unicode scalar value.

Each `char` can represent any valid Unicode character, which includes over a million potential characters. By using 4 bytes (32 bits), Rust ensures that it can accommodate the entire range of Unicode characters efficiently.
