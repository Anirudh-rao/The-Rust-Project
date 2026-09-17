fn main(){
    // Thie Binding lives in the main function
    let long_lived_binding = 1;

    //This is a block and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;
        println!("Inner short:{}", short_lived_binding);
    }

    // Error! `short_lived_binding` doesn't exist in this scope
    //println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line
    println!("Outer long:{}", long_lived_binding);
}