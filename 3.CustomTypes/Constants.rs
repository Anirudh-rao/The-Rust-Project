//  Rust has two type of constants
// Const: Unchangable value
// Static: A possible mutuable version with `static` lifetime
// Static Lifetime is inferred and does not have a specified.

static LANGUAGE : &str = "RUST";
const THRESHOLD : i32 = 10;


fn is_big(n: i32) -> bool{
    // Access constatn in some function
    n > THRESHOLD
}

fn main() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line

}