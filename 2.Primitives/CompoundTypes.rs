fn main(){
    // Tuple in Rust
    // Declared as a comma-seperated list inside paranthesis
    let person:(&str, i32, bool) =("Alice",23,true);

    // Accesing via dot notation
    let name = person.0;

    // Accessing via destructing
    let (name_val, age, is_active) = person;

    // Array
    // Declared inside square brackets .The Type format is [type;length]
    let numbers:[i32;5] =[1,2,3,4,5];

    // Shortcut intialization an arrya with the same value:[value;length]
    let zeros = [0;100];

    // Accesing an element
    let first = numbers[0];

    println!("{}",numbers[2]);
}