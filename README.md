### Structure

Just like other languages, rust starts with the `main()` function. To declare a function in rust, we use `fn FUNCTION_NAME {}`

```rust
fn main() {
	// code here
}
```

### Stdout

To print something to the standard output we use `println!()`

```rust
fn main() {
	println!("Hello, world!");
}
```

### Assembly

Analyzing our program instructions in assembly

![[Pasted image 20241031052651.png]]

Here’s the assembly code you shared, annotated with how it corresponds to the Rust function:

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

The word `let` declares a variable just like in JS, you can also define a type for the variable.

by default, variables in rust are immutable (not to be confused with constants), to declare mutability in a variable we use`mut`: 

```rust
let mut variable shitstorm:bool = true;` 
```

In this example we declared a 8 bit integer, and assign it with the number 64 (max size for 
the memory space we allocated when initialized i8 )

with i8 we are limited to 64, but we can use u8 (unassigned 8 bit integer) to achieve a maximum of 255 (2\*\*8 − 1)

However the smallest value that can be used in an unsigned integer is 0, meaning that it cannot contain negative values

Finally we used string interpolation wiht {} brackets (notice the two methods) to print the variables to stdout

![[Pasted image 20241031202835.png]]

To print the size of anything we can use `std::mem::size_of_val(&first)`

![[Pasted image 20241031202849.png]]


if we try to declare a CHAR using double quotes we get an error: expected `char`, found `&str`

help: if you meant to write a `char` literal, use single quotes,  rustc --explain E0308

In Rust, a `char` is 4 bytes because it represents a Unicode scalar value.

Each `char` can represent any valid Unicode character, which includes over a million potential characters. By using 4 bytes (32 bits), Rust ensures that it can accommodate the entire range of Unicode characters efficiently.