/*
 Numeric Literals can be type annotated by adding the type as suffix. 
 The Type of unsuffixed numeric literals will depemnd on how they are used.
 If no constraints exits , the compiler will use `i32` for integers and
 `f64` for floating point numbers.
 */

fn main(){
    //Suffixed literals , their types are known at intialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;


    //Unsuffixed literals their types depend on how they are used
    let i = 1;
    let f = 1.0;


    //Size_of_val return the size of a variable in bytes
    println!("Size of x in bytes:{}",std::mem::size_of_val(&x));
    println!("Size of y in bytes :{}",std::mem::size_of_val(&y));
    println!("Size of z in bytes :{}",std::mem::size_of_val(&z));
    println!("Size of i in bytes :{}",std::mem::size_of_val(&i));
    println!("Size of f in bytes :{}",std::mem::size_of_val(&f));
    
}
