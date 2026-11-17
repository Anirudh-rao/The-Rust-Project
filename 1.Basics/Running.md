# Basics

Make a new source file and call it main.rs. Rust files always end with the .rs extension. If you’re using more than one word in your filename, the convention is to use an underscore to separate them. For example, use hello_world.rs rather than helloworld.rs.

Now open the main.rs file you just created and enter the code in Listing 1-1.

Filename: main.rs
```
fn main() {
    println!("Hello, world!");
}
```

Save the file and go back to your terminal window in the your directory. On Linux or macOS, enter the following commands to compile and run the file:

```
$ rustc main.rs
$ ./main
Hello, world!
```

Regardless of your operating system, the string Hello, world! should print to the terminal. 

If `Hello, world!` did print, congratulations! You’ve officially written a Rust program. That makes you a Rust programmer—welcome!


## The Anatomy of a Rust Program
Let’s review this “Hello, world!” program in detail. Here’s the first piece of the puzzle:

```
fn main() {

}
```

These lines define a function named `main`. The `main` function is special: It is always the first code that runs in every executable Rust program. Here, the first line declares a function named main that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses `(())`.

The function body is wrapped in `{}`. Rust requires curly brackets around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

The body of the main function holds the following code:
```
println!("Hello, world!");

```
This line does all the work in this little program: It prints text to the screen. There are three important details to notice here.

First, `println!` calls a Rust macro. If it had called a function instead, it would be entered as `println `(without the `!`). Rust macros are a way to write code that generates code to extend Rust syntax. For now, you just need to know that using a`!`Second, you see the "Hello, world!" string. We pass this string as an argument to println!, and the string is printed to the screen.

Third, we end the line with a semicolon (`;`), which indicates that this expression is over, and the next one is ready to begin. Most lines of Rust code end with a semicolon. means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.

If you’re more familiar with a dynamic language, such as Ruby, Python, or JavaScript, you might not be used to compiling and running a program as separate steps. Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed. If you give someone a .rb, .py, or .js file, they need to have a Ruby, Python, or JavaScript implementation installed (respectively). But in those languages, you only need one command to compile and run your program. Everything is a trade-off in language design.

Just compiling with rustc is fine for simple programs, but as your project grows, you’ll want to manage all the options and make it easy to share your code.

