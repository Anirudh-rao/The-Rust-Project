fn main(){
    let shadowed_binding = 1;

    {
        println!("before being shadowed:{}", shadowed_binding);

        // This binding *Shadows* the outer one
        let shadowed_binding = "abc";

        println!("Shadowed in inner block:{}", shadowed_binding);
    }

    println!("outside inner block: {}", shadowed_binding);

    // This binding *shadows* the previous binding
    let shadowed_binding = 2;
    println!("shadowed in outer block: {}", shadowed_binding);
}