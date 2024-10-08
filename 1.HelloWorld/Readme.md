# 1. Hello World 

Lets Deep dive into rust.

## 1. Hello World 
A Sample rust program is given as below
```
// This is a comment, and is ignored by the compiler.
// You can test this code by clicking the "Run" button over there ->
// or if you prefer to use your keyboard, you can use the "Ctrl + Enter"
// shortcut.

// This code is editable, feel free to hack it!
// You can always return to the original code by clicking the "Reset" button ->

// This is the main function.
fn main() {
    // Statements here are executed when the compiled binary is called.

    // Print text to the console.
    println!("Hello World!");
}

```
In the above program we are printing `Hello World` to the console by using a marco
 ``println!``. A Macro in rust is used to print something to the console.

We can rust program from the Terminal using the following commands

```
rustc HelloWorld.rs
```

This will create necessary binaries that can be used to run on the local system.On accessing the `Hello World` Program we the following ouput

```

Hello World!
I'm a Rustacean!

```

## 2. Comments :

Any program requires comments, and Rust supports a few different varieties:

1. Regular comments which are ignored by the compiler:
    
    1. // Line comments which go to the end of the line.
    
    2. /* Block comments which go to the closing delimiter. */

2. Doc comments which are parsed into HTML library documentation:
    
    1. /// Generate library docs for the following item.
    
    2. //! Generate library docs for the enclosing item.


## Formatter Print 

Printing is handeled by a series of macros defined in the `std::fmt`` some of which are :

1. `format!`: write formatter text to string
2. `print!` : same as formate! but the text is printed to the console(io::stdout).
3. `println!`: same as print! but newline is appended.
4. `eprint!`: same as print! but the text is printed to the standard erro(io::stderr).
5. `eprintln!`: same as eprint! but a newline is appended.



