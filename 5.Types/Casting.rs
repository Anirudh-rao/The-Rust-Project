/*
 Casting
 Rust provides no implict type conversion between primitive types but,
 explicit type conversion can be perfomed using `as` keyword.
 Rules of type conversion follows C conventions generally , except in cases where C has undefined behaviour.
*/

//Supress all errors from casts which overflow
#![allow[overflowing_literals]

fn main(){
    let decimal =  65.4321_f32;

    //Error! No Implict Conversion
    //let integer:u8 =  decimal;
    

    //Explicit Conversion
    let integer = decimal as u8;
    let charecter =  integer as char;

    //Error! There are limitation in conversion rules
    //A Float cannot be directly converted to  a char
    //let charecter = decimal as char;
    //
    println!("Casting : {} -> {} -> {}", decimal, integer, charecter);

    //When casting any value to an unsigned type,T
    //T:: max + 1 is aded or subtracted until the alue
    //Fits into the type only when #![allow(overflowing_literals)]
    //lint is specified like above else there will be compiler error
    
    // 1000 already fits in u16;
    println!("10000 as u16 is  {} ",1000 as u16);


    



}
